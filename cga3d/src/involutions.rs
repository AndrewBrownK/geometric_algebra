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

impl AntiDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;

    fn anti_dual(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AntiDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn anti_dual(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn anti_dual(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_dual(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for AntiPlane {
    type Output = Plane;

    fn anti_dual(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_dual(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for AntiScalar {
    type Output = Scalar;

    fn anti_dual(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_dual(self) -> SphereOnOrigin {
        SphereOnOrigin {
            groups: SphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for Circle {
    type Output = Dipole;

    fn anti_dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn anti_dual(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn anti_dual(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn anti_dual(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn anti_dual(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn anti_dual(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiDual for Dilator {
    type Output = MultiVector;

    fn anti_dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], self.group3()[3]]),
                g4: self.group2() * Simd32x3::from(-1.0),
                g5: Simd32x3::from([-self.group3()[0], self.group3()[1], self.group3()[2]]),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiDual for Dipole {
    type Output = Circle;

    fn anti_dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn anti_dual(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn anti_dual(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn anti_dual(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl AntiDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn anti_dual(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn anti_dual(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
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
    type Output = CircleOrthogonalOrigin;

    fn anti_dual(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
    type Output = AntiFlatPointAtOrigin;

    fn anti_dual(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: -self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
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
                g1: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], 0.0]),
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
    type Output = AntiLineAtOrigin;

    fn anti_dual(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups {
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: self.group1() * Simd32x3::from(-1.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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
                g3: Simd32x4::from([-self.group6()[0], -self.group6()[1], -self.group6()[2], self.group8()[3]]),
                g4: self.group7() * Simd32x3::from(-1.0),
                g5: Simd32x3::from([-self.group8()[0], self.group8()[1], self.group8()[2]]),
                g6: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]),
                g7: self.group4(),
                g8: Simd32x4::from([self.group5()[0], self.group5()[1], self.group5()[2], -self.group3()[3]]),
                g9: self.group1() * Simd32x3::from(-1.0),
                g10: self.group2(),
            },
        }
    }
}

impl AntiDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn anti_dual(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn anti_dual(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for NullSphereAtOrigin {
    type Output = Origin;

    fn anti_dual(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl AntiDual for Origin {
    type Output = NullSphereAtOrigin;

    fn anti_dual(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: self.group0() },
        }
    }
}

impl AntiDual for Plane {
    type Output = AntiPlane;

    fn anti_dual(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiDual for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn anti_dual(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups { g0: self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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

impl AntiDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_dual(self) -> SphereAtOrigin {
        SphereAtOrigin {
            groups: SphereAtOriginGroups { g0: self.group0() },
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

impl AntiDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_dual(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl AntiDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn anti_dual(self) -> AntiSphereOnOrigin {
        AntiSphereOnOrigin {
            groups: AntiSphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl AntiReversal for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn anti_reversal(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn anti_reversal(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn anti_reversal(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl AntiReversal for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn anti_reversal(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for AntiPlane {
    type Output = AntiPlane;

    fn anti_reversal(self) -> AntiPlane {
        self
    }
}

impl AntiReversal for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn anti_reversal(self) -> AntiPlaneAtOrigin {
        self
    }
}

impl AntiReversal for AntiScalar {
    type Output = AntiScalar;

    fn anti_reversal(self) -> AntiScalar {
        self
    }
}

impl AntiReversal for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn anti_reversal(self) -> AntiSphereOnOrigin {
        self
    }
}

impl AntiReversal for Circle {
    type Output = Circle;

    fn anti_reversal(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn anti_reversal(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn anti_reversal(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn anti_reversal(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn anti_reversal(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for Dilator {
    type Output = Dilator;

    fn anti_reversal(self) -> Dilator {
        Dilator {
            groups: DilatorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
                g3: self.group3() * Simd32x4::from(-1.0),
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

impl AntiReversal for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn anti_reversal(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
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
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn anti_reversal(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn anti_reversal(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn anti_reversal(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
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
        self
    }
}

impl AntiReversal for Infinity {
    type Output = Infinity;

    fn anti_reversal(self) -> Infinity {
        self
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
                g3: self.group3() * Simd32x4::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x3::from(-1.0),
                g6: self.group6() * Simd32x3::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x4::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl AntiReversal for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn anti_reversal(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn anti_reversal(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn anti_reversal(self) -> NullSphereAtOrigin {
        self
    }
}

impl AntiReversal for Origin {
    type Output = Origin;

    fn anti_reversal(self) -> Origin {
        self
    }
}

impl AntiReversal for Plane {
    type Output = Plane;

    fn anti_reversal(self) -> Plane {
        self
    }
}

impl AntiReversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reversal(self) -> PlaneAtOrigin {
        self
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
        self
    }
}

impl AntiReversal for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn anti_reversal(self) -> RoundPointAtOrigin {
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

impl AntiReversal for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn anti_reversal(self) -> SphereAtOrigin {
        self
    }
}

impl AntiReversal for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn anti_reversal(self) -> SphereOnOrigin {
        self
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

impl Automorphism for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn automorphism(self) -> AntiCircleOnOrigin {
        self
    }
}

impl Automorphism for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn automorphism(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn automorphism(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Automorphism for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn automorphism(self) -> AntiLineAtOrigin {
        self
    }
}

impl Automorphism for AntiPlane {
    type Output = AntiPlane;

    fn automorphism(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn automorphism(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Automorphism for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn automorphism(self) -> AntiSphereOnOrigin {
        AntiSphereOnOrigin {
            groups: AntiSphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for Circle {
    type Output = Circle;

    fn automorphism(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn automorphism(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn automorphism(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn automorphism(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn automorphism(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for Dilator {
    type Output = Dilator;

    fn automorphism(self) -> Dilator {
        Dilator {
            groups: DilatorGroups {
                g0: -self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
                g3: self.group3() * Simd32x4::from(-1.0),
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

impl Automorphism for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn automorphism(self) -> DipoleAligningOrigin {
        self
    }
}

impl Automorphism for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn automorphism(self) -> DipoleAtInfinity {
        self
    }
}

impl Automorphism for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn automorphism(self) -> DipoleAtOrigin {
        self
    }
}

impl Automorphism for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn automorphism(self) -> DipoleOnOrigin {
        self
    }
}

impl Automorphism for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn automorphism(self) -> DipoleOrthogonalOrigin {
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

impl Automorphism for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn automorphism(self) -> FlatPointAtInfinity {
        self
    }
}

impl Automorphism for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn automorphism(self) -> FlatPointAtOrigin {
        self
    }
}

impl Automorphism for Flector {
    type Output = Flector;

    fn automorphism(self) -> Flector {
        self
    }
}

impl Automorphism for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn automorphism(self) -> FlectorAtInfinity {
        self
    }
}

impl Automorphism for Horizon {
    type Output = Horizon;

    fn automorphism(self) -> Horizon {
        self
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
                g6: self.group6() * Simd32x3::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x4::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Automorphism for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn automorphism(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn automorphism(self) -> NullDipoleAtOrigin {
        self
    }
}

impl Automorphism for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn automorphism(self) -> NullSphereAtOrigin {
        self
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
        self
    }
}

impl Automorphism for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn automorphism(self) -> PlaneAtOrigin {
        self
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

impl Automorphism for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn automorphism(self) -> SphereAtOrigin {
        self
    }
}

impl Automorphism for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn automorphism(self) -> SphereOnOrigin {
        self
    }
}

impl Automorphism for Transflector {
    type Output = Transflector;

    fn automorphism(self) -> Transflector {
        self
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

impl Complement for AntiCircleOnOrigin {
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

impl Complement for AntiDipoleOnOrigin {
    type Output = FlatPoint;

    fn complement(self) -> FlatPoint {
        FlatPoint {
            groups: FlatPointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn complement(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Complement for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn complement(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for AntiPlane {
    type Output = SphereOnOrigin;

    fn complement(self) -> SphereOnOrigin {
        SphereOnOrigin {
            groups: SphereOnOriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
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

impl Complement for AntiSphereOnOrigin {
    type Output = Plane;

    fn complement(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl Complement for Circle {
    type Output = Dipole;

    fn complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([-self.group2()[0], self.group2()[1], self.group2()[2]]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([-self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            },
        }
    }
}

impl Complement for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn complement(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group2() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for CircleAtInfinity {
    type Output = Dipole;

    fn complement(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: Simd32x3::from([-self.group1()[0], self.group1()[1], self.group1()[2]]),
                g1: self.group0() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([0.0, 0.0, 0.0, -self.group1()[3]]),
            },
        }
    }
}

impl Complement for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn complement(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for CircleOnOrigin {
    type Output = DipoleAtInfinity;

    fn complement(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
            },
        }
    }
}

impl Complement for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn complement(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: Simd32x3::from([-self.group1()[0], self.group1()[1], self.group1()[2]]),
                g1: Simd32x4::from([-self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
        }
    }
}

impl Complement for Dilator {
    type Output = MultiVector;

    fn complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: self.group3() * Simd32x4::from(-1.0),
                g4: self.group2() * Simd32x3::from(-1.0),
                g5: self.group1() * Simd32x3::from(-1.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Complement for Dipole {
    type Output = Circle;

    fn complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x3::from([-self.group2()[0], self.group2()[1], self.group2()[2]]),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([-self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            },
        }
    }
}

impl Complement for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn complement(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: Simd32x3::from([-self.group1()[0], self.group1()[1], self.group1()[2]]),
                g1: Simd32x4::from([-self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            },
        }
    }
}

impl Complement for DipoleAtInfinity {
    type Output = Circle;

    fn complement(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: Simd32x3::from([-self.group1()[0], self.group1()[1], self.group1()[2]]),
                g1: self.group0() * Simd32x3::from(-1.0),
                g2: Simd32x4::from([0.0, 0.0, 0.0, -self.group1()[3]]),
            },
        }
    }
}

impl Complement for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn complement(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for DipoleOnOrigin {
    type Output = CircleOrthogonalOrigin;

    fn complement(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn complement(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group2() * Simd32x3::from(-1.0),
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
    type Output = AntiDipoleOnOrigin;

    fn complement(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Complement for FlatPointAtInfinity {
    type Output = NullCircleAtOrigin;

    fn complement(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for FlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn complement(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: -self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from([0.0, 0.0, 0.0, -self.group0()[3]]),
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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
    type Output = NullSphereAtOrigin;

    fn complement(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for Line {
    type Output = AntiCircleOnOrigin;

    fn complement(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for LineAtInfinity {
    type Output = NullDipoleAtOrigin;

    fn complement(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for LineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn complement(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups {
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
                g3: Simd32x4::from([-self.group1()[0], -self.group1()[1], -self.group1()[2], 0.0]),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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
                g3: self.group8() * Simd32x4::from(-1.0),
                g4: self.group7() * Simd32x3::from(-1.0),
                g5: self.group6() * Simd32x3::from(-1.0),
                g6: self.group5() * Simd32x3::from(-1.0),
                g7: self.group4() * Simd32x3::from(-1.0),
                g8: self.group3() * Simd32x4::from(-1.0),
                g9: self.group1(),
                g10: swizzle!(self.group2(), 1, 0),
            },
        }
    }
}

impl Complement for NullCircleAtOrigin {
    type Output = FlatPointAtInfinity;

    fn complement(self) -> FlatPointAtInfinity {
        FlatPointAtInfinity {
            groups: FlatPointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for NullDipoleAtOrigin {
    type Output = LineAtInfinity;

    fn complement(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Complement for NullSphereAtOrigin {
    type Output = Infinity;

    fn complement(self) -> Infinity {
        Infinity {
            groups: InfinityGroups { g0: self.group0() },
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
    type Output = AntiSphereOnOrigin;

    fn complement(self) -> AntiSphereOnOrigin {
        AntiSphereOnOrigin {
            groups: AntiSphereOnOriginGroups { g0: self.group0() },
        }
    }
}

impl Complement for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn complement(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups { g0: self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from([-self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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

impl Complement for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn complement(self) -> SphereAtOrigin {
        SphereAtOrigin {
            groups: SphereAtOriginGroups {
                g0: swizzle!(self.group0(), 1, 0),
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

impl Complement for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn complement(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl Complement for SphereOnOrigin {
    type Output = AntiPlane;

    fn complement(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups { g0: self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: self.group0() * Simd32x3::from(-1.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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
                g3: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl ConformalConjugate for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn conformal_conjugate(self) -> AntiCircleOnOrigin {
        self
    }
}

impl ConformalConjugate for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn conformal_conjugate(self) -> AntiDipoleOnOrigin {
        self
    }
}

impl ConformalConjugate for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn conformal_conjugate(self) -> AntiFlatPointAtOrigin {
        self
    }
}

impl ConformalConjugate for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn conformal_conjugate(self) -> AntiLineAtOrigin {
        self
    }
}

impl ConformalConjugate for AntiPlane {
    type Output = AntiPlane;

    fn conformal_conjugate(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn conformal_conjugate(self) -> AntiPlaneAtOrigin {
        self
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

impl ConformalConjugate for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn conformal_conjugate(self) -> AntiSphereOnOrigin {
        self
    }
}

impl ConformalConjugate for Circle {
    type Output = Circle;

    fn conformal_conjugate(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl ConformalConjugate for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn conformal_conjugate(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
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
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl ConformalConjugate for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn conformal_conjugate(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn conformal_conjugate(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn conformal_conjugate(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl ConformalConjugate for Dilator {
    type Output = Dilator;

    fn conformal_conjugate(self) -> Dilator {
        Dilator {
            groups: DilatorGroups {
                g0: -self.group0(),
                g1: self.group1(),
                g2: self.group2() * Simd32x3::from(-1.0),
                g3: self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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

impl ConformalConjugate for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn conformal_conjugate(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from(-1.0),
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
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn conformal_conjugate(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl ConformalConjugate for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn conformal_conjugate(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn conformal_conjugate(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() * Simd32x3::from(-1.0),
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
                g3: self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
                g4: self.group4(),
                g5: self.group5() * Simd32x3::from(-1.0),
                g6: self.group6(),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
                g9: self.group9() * Simd32x3::from(-1.0),
                g10: self.group10() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn conformal_conjugate(self) -> NullCircleAtOrigin {
        self
    }
}

impl ConformalConjugate for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn conformal_conjugate(self) -> NullDipoleAtOrigin {
        self
    }
}

impl ConformalConjugate for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn conformal_conjugate(self) -> NullSphereAtOrigin {
        self
    }
}

impl ConformalConjugate for Origin {
    type Output = Origin;

    fn conformal_conjugate(self) -> Origin {
        self
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

impl ConformalConjugate for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn conformal_conjugate(self) -> SphereAtOrigin {
        SphereAtOrigin {
            groups: SphereAtOriginGroups {
                g0: self.group0() * Simd32x2::from([1.0, -1.0]),
            },
        }
    }
}

impl ConformalConjugate for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn conformal_conjugate(self) -> SphereOnOrigin {
        SphereOnOrigin {
            groups: SphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
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

impl Conjugation for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn conjugation(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn conjugation(self) -> AntiDipoleOnOrigin {
        self
    }
}

impl Conjugation for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn conjugation(self) -> AntiFlatPointAtOrigin {
        self
    }
}

impl Conjugation for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn conjugation(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for AntiPlane {
    type Output = AntiPlane;

    fn conjugation(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn conjugation(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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

impl Conjugation for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn conjugation(self) -> AntiSphereOnOrigin {
        AntiSphereOnOrigin {
            groups: AntiSphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for Circle {
    type Output = Circle;

    fn conjugation(self) -> Circle {
        self
    }
}

impl Conjugation for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn conjugation(self) -> CircleAligningOrigin {
        self
    }
}

impl Conjugation for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn conjugation(self) -> CircleAtInfinity {
        self
    }
}

impl Conjugation for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn conjugation(self) -> CircleAtOrigin {
        self
    }
}

impl Conjugation for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn conjugation(self) -> CircleOnOrigin {
        self
    }
}

impl Conjugation for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn conjugation(self) -> CircleOrthogonalOrigin {
        self
    }
}

impl Conjugation for Dilator {
    type Output = Dilator;

    fn conjugation(self) -> Dilator {
        Dilator {
            groups: DilatorGroups {
                g0: -self.group0(),
                g1: self.group1(),
                g2: self.group2(),
                g3: self.group3(),
            },
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

impl Conjugation for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn conjugation(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
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
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn conjugation(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn conjugation(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn conjugation(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
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
        self
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
        self
    }
}

impl Conjugation for LineAtInfinity {
    type Output = LineAtInfinity;

    fn conjugation(self) -> LineAtInfinity {
        self
    }
}

impl Conjugation for LineAtOrigin {
    type Output = LineAtOrigin;

    fn conjugation(self) -> LineAtOrigin {
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
                g3: self.group3() * Simd32x4::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x3::from(-1.0),
                g6: self.group6(),
                g7: self.group7(),
                g8: self.group8(),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Conjugation for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn conjugation(self) -> NullCircleAtOrigin {
        self
    }
}

impl Conjugation for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn conjugation(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn conjugation(self) -> NullSphereAtOrigin {
        self
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
        self
    }
}

impl Conjugation for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn conjugation(self) -> PlaneAtOrigin {
        self
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

impl Conjugation for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn conjugation(self) -> SphereAtOrigin {
        self
    }
}

impl Conjugation for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn conjugation(self) -> SphereOnOrigin {
        self
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

impl DoubleComplement for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn double_complement(self) -> AntiCircleOnOrigin {
        self
    }
}

impl DoubleComplement for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn double_complement(self) -> AntiDipoleOnOrigin {
        self
    }
}

impl DoubleComplement for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn double_complement(self) -> AntiFlatPointAtOrigin {
        self
    }
}

impl DoubleComplement for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn double_complement(self) -> AntiLineAtOrigin {
        self
    }
}

impl DoubleComplement for AntiPlane {
    type Output = AntiPlane;

    fn double_complement(self) -> AntiPlane {
        self
    }
}

impl DoubleComplement for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn double_complement(self) -> AntiPlaneAtOrigin {
        self
    }
}

impl DoubleComplement for AntiScalar {
    type Output = AntiScalar;

    fn double_complement(self) -> AntiScalar {
        self
    }
}

impl DoubleComplement for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn double_complement(self) -> AntiSphereOnOrigin {
        self
    }
}

impl DoubleComplement for Circle {
    type Output = Circle;

    fn double_complement(self) -> Circle {
        self
    }
}

impl DoubleComplement for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn double_complement(self) -> CircleAligningOrigin {
        self
    }
}

impl DoubleComplement for CircleAtInfinity {
    type Output = CircleAtInfinity;

    fn double_complement(self) -> CircleAtInfinity {
        self
    }
}

impl DoubleComplement for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn double_complement(self) -> CircleAtOrigin {
        self
    }
}

impl DoubleComplement for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn double_complement(self) -> CircleOnOrigin {
        self
    }
}

impl DoubleComplement for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn double_complement(self) -> CircleOrthogonalOrigin {
        self
    }
}

impl DoubleComplement for Dilator {
    type Output = Dilator;

    fn double_complement(self) -> Dilator {
        self
    }
}

impl DoubleComplement for Dipole {
    type Output = Dipole;

    fn double_complement(self) -> Dipole {
        self
    }
}

impl DoubleComplement for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn double_complement(self) -> DipoleAligningOrigin {
        self
    }
}

impl DoubleComplement for DipoleAtInfinity {
    type Output = DipoleAtInfinity;

    fn double_complement(self) -> DipoleAtInfinity {
        self
    }
}

impl DoubleComplement for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn double_complement(self) -> DipoleAtOrigin {
        self
    }
}

impl DoubleComplement for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn double_complement(self) -> DipoleOnOrigin {
        self
    }
}

impl DoubleComplement for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn double_complement(self) -> DipoleOrthogonalOrigin {
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

impl DoubleComplement for FlatPointAtInfinity {
    type Output = FlatPointAtInfinity;

    fn double_complement(self) -> FlatPointAtInfinity {
        self
    }
}

impl DoubleComplement for FlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn double_complement(self) -> FlatPointAtOrigin {
        self
    }
}

impl DoubleComplement for Flector {
    type Output = Flector;

    fn double_complement(self) -> Flector {
        self
    }
}

impl DoubleComplement for FlectorAtInfinity {
    type Output = FlectorAtInfinity;

    fn double_complement(self) -> FlectorAtInfinity {
        self
    }
}

impl DoubleComplement for Horizon {
    type Output = Horizon;

    fn double_complement(self) -> Horizon {
        self
    }
}

impl DoubleComplement for Infinity {
    type Output = Infinity;

    fn double_complement(self) -> Infinity {
        self
    }
}

impl DoubleComplement for Line {
    type Output = Line;

    fn double_complement(self) -> Line {
        self
    }
}

impl DoubleComplement for LineAtInfinity {
    type Output = LineAtInfinity;

    fn double_complement(self) -> LineAtInfinity {
        self
    }
}

impl DoubleComplement for LineAtOrigin {
    type Output = LineAtOrigin;

    fn double_complement(self) -> LineAtOrigin {
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

impl DoubleComplement for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn double_complement(self) -> NullCircleAtOrigin {
        self
    }
}

impl DoubleComplement for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn double_complement(self) -> NullDipoleAtOrigin {
        self
    }
}

impl DoubleComplement for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn double_complement(self) -> NullSphereAtOrigin {
        self
    }
}

impl DoubleComplement for Origin {
    type Output = Origin;

    fn double_complement(self) -> Origin {
        self
    }
}

impl DoubleComplement for Plane {
    type Output = Plane;

    fn double_complement(self) -> Plane {
        self
    }
}

impl DoubleComplement for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn double_complement(self) -> PlaneAtOrigin {
        self
    }
}

impl DoubleComplement for Rotor {
    type Output = Rotor;

    fn double_complement(self) -> Rotor {
        self
    }
}

impl DoubleComplement for RoundPoint {
    type Output = RoundPoint;

    fn double_complement(self) -> RoundPoint {
        self
    }
}

impl DoubleComplement for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn double_complement(self) -> RoundPointAtOrigin {
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

impl DoubleComplement for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn double_complement(self) -> SphereAtOrigin {
        self
    }
}

impl DoubleComplement for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn double_complement(self) -> SphereOnOrigin {
        self
    }
}

impl DoubleComplement for Transflector {
    type Output = Transflector;

    fn double_complement(self) -> Transflector {
        self
    }
}

impl DoubleComplement for Translator {
    type Output = Translator;

    fn double_complement(self) -> Translator {
        self
    }
}

impl Dual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;

    fn dual(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn dual(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for AntiFlatPointAtOrigin {
    type Output = FlatPointAtOrigin;

    fn dual(self) -> FlatPointAtOrigin {
        FlatPointAtOrigin {
            groups: FlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Dual for AntiLineAtOrigin {
    type Output = LineAtOrigin;

    fn dual(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for AntiPlane {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for AntiPlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn dual(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
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

impl Dual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;

    fn dual(self) -> SphereOnOrigin {
        SphereOnOrigin {
            groups: SphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for Circle {
    type Output = Dipole;

    fn dual(self) -> Dipole {
        Dipole {
            groups: DipoleGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn dual(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
                g2: self.group2(),
            },
        }
    }
}

impl Dual for CircleAtInfinity {
    type Output = DipoleAtInfinity;

    fn dual(self) -> DipoleAtInfinity {
        DipoleAtInfinity {
            groups: DipoleAtInfinityGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for CircleAtOrigin {
    type Output = DipoleAtOrigin;

    fn dual(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Dual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn dual(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group0(),
                g1: self.group1(),
            },
        }
    }
}

impl Dual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;

    fn dual(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Dual for Dilator {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: Simd32x2::from([-self.group0(), 0.0]),
                g1: Simd32x3::from(0.0),
                g2: Simd32x2::from(0.0),
                g3: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], -self.group3()[3]]),
                g4: self.group2(),
                g5: Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Dual for Dipole {
    type Output = Circle;

    fn dual(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;

    fn dual(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for DipoleAtInfinity {
    type Output = CircleAtInfinity;

    fn dual(self) -> CircleAtInfinity {
        CircleAtInfinity {
            groups: CircleAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for DipoleAtOrigin {
    type Output = CircleAtOrigin;

    fn dual(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn dual(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;

    fn dual(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
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
    type Output = CircleOrthogonalOrigin;

    fn dual(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: Simd32x3::from(0.0),
                g1: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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
    type Output = AntiFlatPointAtOrigin;

    fn dual(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
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
                g1: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
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
    type Output = AntiLineAtOrigin;

    fn dual(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups { g0: self.group0() },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: self.group1(),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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
                g3: Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], -self.group8()[3]]),
                g4: self.group7(),
                g5: Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]),
                g6: Simd32x3::from([-self.group3()[0], self.group3()[1], self.group3()[2]]),
                g7: self.group4() * Simd32x3::from(-1.0),
                g8: Simd32x4::from([-self.group5()[0], -self.group5()[1], -self.group5()[2], self.group3()[3]]),
                g9: self.group1(),
                g10: self.group2() * Simd32x2::from(-1.0),
            },
        }
    }
}

impl Dual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn dual(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Dual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn dual(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for NullSphereAtOrigin {
    type Output = Origin;

    fn dual(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl Dual for Origin {
    type Output = NullSphereAtOrigin;

    fn dual(self) -> NullSphereAtOrigin {
        NullSphereAtOrigin {
            groups: NullSphereAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Dual for Plane {
    type Output = AntiPlane;

    fn dual(self) -> AntiPlane {
        AntiPlane {
            groups: AntiPlaneGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Dual for PlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn dual(self) -> AntiPlaneAtOrigin {
        AntiPlaneAtOrigin {
            groups: AntiPlaneAtOriginGroups {
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
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

impl Dual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;

    fn dual(self) -> SphereAtOrigin {
        SphereAtOrigin {
            groups: SphereAtOriginGroups {
                g0: self.group0() * Simd32x2::from(-1.0),
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

impl Dual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;

    fn dual(self) -> RoundPointAtOrigin {
        RoundPointAtOrigin {
            groups: RoundPointAtOriginGroups { g0: self.group0() },
        }
    }
}

impl Dual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn dual(self) -> AntiSphereOnOrigin {
        AntiSphereOnOrigin {
            groups: AntiSphereOnOriginGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            },
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from(0.0),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from([-self.group0()[0], -self.group0()[1], -self.group0()[2], 0.0]),
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
                g3: Simd32x4::from(0.0),
                g4: Simd32x3::from(0.0),
                g5: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
                g6: Simd32x3::from(0.0),
                g7: Simd32x3::from(0.0),
                g8: Simd32x4::from(0.0),
                g9: Simd32x3::from(0.0),
                g10: Simd32x2::from(0.0),
            },
        }
    }
}

impl Reversal for AntiCircleOnOrigin {
    type Output = AntiCircleOnOrigin;

    fn reversal(self) -> AntiCircleOnOrigin {
        AntiCircleOnOrigin {
            groups: AntiCircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for AntiDipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;

    fn reversal(self) -> AntiDipoleOnOrigin {
        AntiDipoleOnOrigin {
            groups: AntiDipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for AntiFlatPointAtOrigin {
    type Output = AntiFlatPointAtOrigin;

    fn reversal(self) -> AntiFlatPointAtOrigin {
        AntiFlatPointAtOrigin {
            groups: AntiFlatPointAtOriginGroups { g0: -self.group0() },
        }
    }
}

impl Reversal for AntiLineAtOrigin {
    type Output = AntiLineAtOrigin;

    fn reversal(self) -> AntiLineAtOrigin {
        AntiLineAtOrigin {
            groups: AntiLineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for AntiPlane {
    type Output = AntiPlane;

    fn reversal(self) -> AntiPlane {
        self
    }
}

impl Reversal for AntiPlaneAtOrigin {
    type Output = AntiPlaneAtOrigin;

    fn reversal(self) -> AntiPlaneAtOrigin {
        self
    }
}

impl Reversal for AntiScalar {
    type Output = AntiScalar;

    fn reversal(self) -> AntiScalar {
        self
    }
}

impl Reversal for AntiSphereOnOrigin {
    type Output = AntiSphereOnOrigin;

    fn reversal(self) -> AntiSphereOnOrigin {
        self
    }
}

impl Reversal for Circle {
    type Output = Circle;

    fn reversal(self) -> Circle {
        Circle {
            groups: CircleGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleAligningOrigin {
    type Output = CircleAligningOrigin;

    fn reversal(self) -> CircleAligningOrigin {
        CircleAligningOrigin {
            groups: CircleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
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
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleAtOrigin {
    type Output = CircleAtOrigin;

    fn reversal(self) -> CircleAtOrigin {
        CircleAtOrigin {
            groups: CircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleOnOrigin {
    type Output = CircleOnOrigin;

    fn reversal(self) -> CircleOnOrigin {
        CircleOnOrigin {
            groups: CircleOnOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for CircleOrthogonalOrigin {
    type Output = CircleOrthogonalOrigin;

    fn reversal(self) -> CircleOrthogonalOrigin {
        CircleOrthogonalOrigin {
            groups: CircleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for Dilator {
    type Output = Dilator;

    fn reversal(self) -> Dilator {
        Dilator {
            groups: DilatorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
                g3: self.group3() * Simd32x4::from(-1.0),
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

impl Reversal for DipoleAligningOrigin {
    type Output = DipoleAligningOrigin;

    fn reversal(self) -> DipoleAligningOrigin {
        DipoleAligningOrigin {
            groups: DipoleAligningOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
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
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleAtOrigin {
    type Output = DipoleAtOrigin;

    fn reversal(self) -> DipoleAtOrigin {
        DipoleAtOrigin {
            groups: DipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleOnOrigin {
    type Output = DipoleOnOrigin;

    fn reversal(self) -> DipoleOnOrigin {
        DipoleOnOrigin {
            groups: DipoleOnOriginGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Reversal for DipoleOrthogonalOrigin {
    type Output = DipoleOrthogonalOrigin;

    fn reversal(self) -> DipoleOrthogonalOrigin {
        DipoleOrthogonalOrigin {
            groups: DipoleOrthogonalOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
                g1: self.group1() * Simd32x3::from(-1.0),
                g2: self.group2() * Simd32x3::from(-1.0),
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
        self
    }
}

impl Reversal for Infinity {
    type Output = Infinity;

    fn reversal(self) -> Infinity {
        self
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
                g3: self.group3() * Simd32x4::from(-1.0),
                g4: self.group4() * Simd32x3::from(-1.0),
                g5: self.group5() * Simd32x3::from(-1.0),
                g6: self.group6() * Simd32x3::from(-1.0),
                g7: self.group7() * Simd32x3::from(-1.0),
                g8: self.group8() * Simd32x4::from(-1.0),
                g9: self.group9(),
                g10: self.group10(),
            },
        }
    }
}

impl Reversal for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;

    fn reversal(self) -> NullCircleAtOrigin {
        NullCircleAtOrigin {
            groups: NullCircleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for NullDipoleAtOrigin {
    type Output = NullDipoleAtOrigin;

    fn reversal(self) -> NullDipoleAtOrigin {
        NullDipoleAtOrigin {
            groups: NullDipoleAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Reversal for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;

    fn reversal(self) -> NullSphereAtOrigin {
        self
    }
}

impl Reversal for Origin {
    type Output = Origin;

    fn reversal(self) -> Origin {
        self
    }
}

impl Reversal for Plane {
    type Output = Plane;

    fn reversal(self) -> Plane {
        self
    }
}

impl Reversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reversal(self) -> PlaneAtOrigin {
        self
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
        self
    }
}

impl Reversal for RoundPointAtOrigin {
    type Output = RoundPointAtOrigin;

    fn reversal(self) -> RoundPointAtOrigin {
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

impl Reversal for SphereAtOrigin {
    type Output = SphereAtOrigin;

    fn reversal(self) -> SphereAtOrigin {
        self
    }
}

impl Reversal for SphereOnOrigin {
    type Output = SphereOnOrigin;

    fn reversal(self) -> SphereOnOrigin {
        self
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
