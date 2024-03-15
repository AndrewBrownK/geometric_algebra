#![allow(clippy::assign_op_pattern)]
use geometric_algebra::{rga3d::*, simd::*, *};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Negates elements with `grade % 2 == 1`
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
/// Also called transpose
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Reverses
pub trait Reversal {
    type Output;
    fn reversal(self) -> Self::Output;
}

/// Negates elements with `grade % 4 >= 2`
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

/// Right Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait RightComplement {
    type Output;
    fn right_complement(self) -> Self::Output;
}

/// Left Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait LeftComplement {
    type Output;
    fn left_complement(self) -> Self::Output;
}

/// Double Complement
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Complements
pub trait DoubleComplement {
    type Output;
    fn double_complement(self) -> Self::Output;
}

impl AntiReversal for AntiScalar {
    type Output = AntiScalar;

    fn anti_reversal(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Flector {
    type Output = Flector;

    fn anti_reversal(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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

impl AntiReversal for Line {
    type Output = Line;

    fn anti_reversal(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
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
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiReversal for Magnitude {
    type Output = Magnitude;

    fn anti_reversal(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
        }
    }
}

impl AntiReversal for Motor {
    type Output = Motor;

    fn anti_reversal(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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
                g1: self.group1() * Simd32x4::from(-1.0),
                g2: self.group2() * Simd32x3::from([-1.0, 1.0, -1.0]),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiReversal for Origin {
    type Output = Origin;

    fn anti_reversal(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl AntiReversal for Plane {
    type Output = Plane;

    fn anti_reversal(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
        }
    }
}

impl AntiReversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_reversal(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl AntiReversal for Point {
    type Output = Point;

    fn anti_reversal(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl AntiReversal for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_reversal(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl AntiReversal for Rotor {
    type Output = Rotor;

    fn anti_reversal(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
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
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Flector {
    type Output = Flector;

    fn automorphism(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Automorphism for Horizon {
    type Output = Horizon;

    fn automorphism(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
        }
    }
}

impl Automorphism for Line {
    type Output = Line;

    fn automorphism(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for LineAtInfinity {
    type Output = LineAtInfinity;

    fn automorphism(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for LineAtOrigin {
    type Output = LineAtOrigin;

    fn automorphism(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Automorphism for Magnitude {
    type Output = Magnitude;

    fn automorphism(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
        }
    }
}

impl Automorphism for Motor {
    type Output = Motor;

    fn automorphism(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
                g1: self.group1(),
            },
        }
    }
}

impl Automorphism for MultiVector {
    type Output = MultiVector;

    fn automorphism(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from(-1.0),
                g2: self.group2() * Simd32x3::from([1.0, -1.0, 1.0]),
                g3: self.group3(),
                g4: self.group4() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
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
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Automorphism for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn automorphism(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Automorphism for Point {
    type Output = Point;

    fn automorphism(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Automorphism for PointAtInfinity {
    type Output = PointAtInfinity;

    fn automorphism(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Automorphism for Rotor {
    type Output = Rotor;

    fn automorphism(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, 1.0]),
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

impl Automorphism for Translator {
    type Output = Translator;

    fn automorphism(self) -> Translator {
        Translator {
            groups: TranslatorGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for AntiScalar {
    type Output = AntiScalar;

    fn conjugation(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Flector {
    type Output = Flector;

    fn conjugation(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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

impl Conjugation for Line {
    type Output = Line;

    fn conjugation(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for LineAtInfinity {
    type Output = LineAtInfinity;

    fn conjugation(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for LineAtOrigin {
    type Output = LineAtOrigin;

    fn conjugation(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Conjugation for Magnitude {
    type Output = Magnitude;

    fn conjugation(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
        }
    }
}

impl Conjugation for Motor {
    type Output = Motor;

    fn conjugation(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
                g1: self.group1() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for MultiVector {
    type Output = MultiVector;

    fn conjugation(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from(-1.0),
                g2: self.group2() * Simd32x3::from([-1.0, 1.0, -1.0]),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Conjugation for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn conjugation(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Conjugation for Point {
    type Output = Point;

    fn conjugation(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Conjugation for PointAtInfinity {
    type Output = PointAtInfinity;

    fn conjugation(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Conjugation for Rotor {
    type Output = Rotor;

    fn conjugation(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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

impl Conjugation for Translator {
    type Output = Translator;

    fn conjugation(self) -> Translator {
        Translator {
            groups: TranslatorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
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

impl DoubleComplement for Flector {
    type Output = Flector;

    fn double_complement(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
                g1: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl DoubleComplement for Horizon {
    type Output = Horizon;

    fn double_complement(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
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

impl DoubleComplement for Magnitude {
    type Output = Magnitude;

    fn double_complement(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
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
                g1: self.group1() * Simd32x4::from(-1.0),
                g2: self.group2(),
                g3: self.group3(),
                g4: self.group4() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl DoubleComplement for Origin {
    type Output = Origin;

    fn double_complement(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl DoubleComplement for Plane {
    type Output = Plane;

    fn double_complement(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl DoubleComplement for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn double_complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl DoubleComplement for Point {
    type Output = Point;

    fn double_complement(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl DoubleComplement for PointAtInfinity {
    type Output = PointAtInfinity;

    fn double_complement(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
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

impl DoubleComplement for Scalar {
    type Output = Scalar;

    fn double_complement(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
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
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl Dual for Flector {
    type Output = Flector;

    fn dual(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group1() * Simd32x4::from(-1.0),
                g1: self.group0(),
            },
        }
    }
}

impl Dual for Horizon {
    type Output = Origin;

    fn dual(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl Dual for Line {
    type Output = Line;

    fn dual(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for LineAtInfinity {
    type Output = LineAtOrigin;

    fn dual(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for LineAtOrigin {
    type Output = LineAtInfinity;

    fn dual(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for Magnitude {
    type Output = Magnitude;

    fn dual(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl Dual for MultiVector {
    type Output = MultiVector;

    fn dual(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0),
                g1: self.group4() * Simd32x4::from(-1.0),
                g2: self.group3() * Simd32x3::from(-1.0),
                g3: self.group2() * Simd32x3::from(-1.0),
                g4: self.group1(),
            },
        }
    }
}

impl Dual for Origin {
    type Output = Horizon;

    fn dual(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl Dual for Plane {
    type Output = Point;

    fn dual(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl Dual for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn dual(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl Dual for Point {
    type Output = Plane;

    fn dual(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl Dual for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn dual(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
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

impl LeftComplement for AntiScalar {
    type Output = Scalar;

    fn left_complement(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl LeftComplement for Flector {
    type Output = Flector;

    fn left_complement(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group1(),
                g1: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl LeftComplement for Horizon {
    type Output = Origin;

    fn left_complement(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: self.group0() },
        }
    }
}

impl LeftComplement for Line {
    type Output = Line;

    fn left_complement(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl LeftComplement for LineAtInfinity {
    type Output = LineAtOrigin;

    fn left_complement(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl LeftComplement for LineAtOrigin {
    type Output = LineAtInfinity;

    fn left_complement(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl LeftComplement for Magnitude {
    type Output = Magnitude;

    fn left_complement(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl LeftComplement for MultiVector {
    type Output = MultiVector;

    fn left_complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0),
                g1: self.group4(),
                g2: self.group3() * Simd32x3::from(-1.0),
                g3: self.group2() * Simd32x3::from(-1.0),
                g4: self.group1() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl LeftComplement for Origin {
    type Output = Horizon;

    fn left_complement(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
        }
    }
}

impl LeftComplement for Plane {
    type Output = Point;

    fn left_complement(self) -> Point {
        Point {
            groups: PointGroups { g0: self.group0() },
        }
    }
}

impl LeftComplement for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn left_complement(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl LeftComplement for Point {
    type Output = Plane;

    fn left_complement(self) -> Plane {
        Plane {
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl LeftComplement for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn left_complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl LeftComplement for Scalar {
    type Output = AntiScalar;

    fn left_complement(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
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

impl Reversal for Flector {
    type Output = Flector;

    fn reversal(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group0(),
                g1: self.group1() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Reversal for Horizon {
    type Output = Horizon;

    fn reversal(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: -self.group0() },
        }
    }
}

impl Reversal for Line {
    type Output = Line;

    fn reversal(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
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
                g0: self.group0() * Simd32x3::from([-1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Reversal for Magnitude {
    type Output = Magnitude;

    fn reversal(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Motor {
    type Output = Motor;

    fn reversal(self) -> Motor {
        Motor {
            groups: MotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
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
                g2: self.group2() * Simd32x3::from([-1.0, 1.0, -1.0]),
                g3: self.group3() * Simd32x3::from(-1.0),
                g4: self.group4() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
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
            groups: PlaneGroups {
                g0: self.group0() * Simd32x4::from([1.0, -1.0, 1.0, -1.0]),
            },
        }
    }
}

impl Reversal for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn reversal(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups {
                g0: self.group0() * Simd32x3::from([1.0, -1.0, 1.0]),
            },
        }
    }
}

impl Reversal for Point {
    type Output = Point;

    fn reversal(self) -> Point {
        Point {
            groups: PointGroups { g0: self.group0() },
        }
    }
}

impl Reversal for PointAtInfinity {
    type Output = PointAtInfinity;

    fn reversal(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups { g0: self.group0() },
        }
    }
}

impl Reversal for Rotor {
    type Output = Rotor;

    fn reversal(self) -> Rotor {
        Rotor {
            groups: RotorGroups {
                g0: self.group0() * Simd32x4::from([-1.0, 1.0, -1.0, 1.0]),
            },
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

impl RightComplement for AntiScalar {
    type Output = Scalar;

    fn right_complement(self) -> Scalar {
        Scalar {
            groups: ScalarGroups { g0: self.group0() },
        }
    }
}

impl RightComplement for Flector {
    type Output = Flector;

    fn right_complement(self) -> Flector {
        Flector {
            groups: FlectorGroups {
                g0: self.group1() * Simd32x4::from(-1.0),
                g1: self.group0(),
            },
        }
    }
}

impl RightComplement for Horizon {
    type Output = Origin;

    fn right_complement(self) -> Origin {
        Origin {
            groups: OriginGroups { g0: -self.group0() },
        }
    }
}

impl RightComplement for Line {
    type Output = Line;

    fn right_complement(self) -> Line {
        Line {
            groups: LineGroups {
                g0: self.group1() * Simd32x3::from(-1.0),
                g1: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl RightComplement for LineAtInfinity {
    type Output = LineAtOrigin;

    fn right_complement(self) -> LineAtOrigin {
        LineAtOrigin {
            groups: LineAtOriginGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl RightComplement for LineAtOrigin {
    type Output = LineAtInfinity;

    fn right_complement(self) -> LineAtInfinity {
        LineAtInfinity {
            groups: LineAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl RightComplement for Magnitude {
    type Output = Magnitude;

    fn right_complement(self) -> Magnitude {
        Magnitude {
            groups: MagnitudeGroups {
                g0: swizzle!(self.group0(), 1, 0),
            },
        }
    }
}

impl RightComplement for MultiVector {
    type Output = MultiVector;

    fn right_complement(self) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {
                g0: swizzle!(self.group0(), 1, 0),
                g1: self.group4() * Simd32x4::from(-1.0),
                g2: self.group3() * Simd32x3::from(-1.0),
                g3: self.group2() * Simd32x3::from(-1.0),
                g4: self.group1(),
            },
        }
    }
}

impl RightComplement for Origin {
    type Output = Horizon;

    fn right_complement(self) -> Horizon {
        Horizon {
            groups: HorizonGroups { g0: self.group0() },
        }
    }
}

impl RightComplement for Plane {
    type Output = Point;

    fn right_complement(self) -> Point {
        Point {
            groups: PointGroups {
                g0: self.group0() * Simd32x4::from(-1.0),
            },
        }
    }
}

impl RightComplement for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn right_complement(self) -> PointAtInfinity {
        PointAtInfinity {
            groups: PointAtInfinityGroups {
                g0: self.group0() * Simd32x3::from(-1.0),
            },
        }
    }
}

impl RightComplement for Point {
    type Output = Plane;

    fn right_complement(self) -> Plane {
        Plane {
            groups: PlaneGroups { g0: self.group0() },
        }
    }
}

impl RightComplement for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn right_complement(self) -> PlaneAtOrigin {
        PlaneAtOrigin {
            groups: PlaneAtOriginGroups { g0: self.group0() },
        }
    }
}

impl RightComplement for Scalar {
    type Output = AntiScalar;

    fn right_complement(self) -> AntiScalar {
        AntiScalar {
            groups: AntiScalarGroups { g0: self.group0() },
        }
    }
}
