
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;


/// Left Interior Product
/// Synonyms included: LeftContraction
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait LeftContraction<T> {
    type Output;
    fn left_contraction(self, other: T) -> Self::Output;
}


/// Left Interior Anti-Product
/// Synonyms included: LeftAntiContraction
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait LeftAntiContraction<T> {
    type Output;
    fn left_anti_contraction(self, other: T) -> Self::Output;
}


/// Right Interior Product
/// Synonyms included: RightContraction
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait RightContraction<T> {
    type Output;
    fn right_contraction(self, other: T) -> Self::Output;
}


/// Right Interior Anti-Product
/// Synonyms included: RightAntiContraction
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Interior_products
pub trait RightAntiContraction<T> {
    type Output;
    fn right_anti_contraction(self, other: T) -> Self::Output;
}

impl LeftAntiContraction<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for AntiScalar {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<Horizon> for AntiScalar {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for AntiScalar {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn left_anti_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Motor> for AntiScalar {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftAntiContraction<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl LeftAntiContraction<Origin> for AntiScalar {
    type Output = Origin;

    fn left_anti_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Plane> for AntiScalar {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for AntiScalar {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Rotor> for AntiScalar {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Scalar> for AntiScalar {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Translator> for AntiScalar {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Flector {
    type Output = Motor;

    fn left_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Line> for Flector {
    type Output = Plane;

    fn left_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Flector {
    type Output = Horizon;

    fn left_anti_contraction(self, other: LineAtInfinity) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Flector {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for Flector {
    type Output = Flector;

    fn left_anti_contraction(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g1: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Motor> for Flector {
    type Output = Plane;

    fn left_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group3()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Flector {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Origin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(other.group0()) * Simd32x4::from(-1.0) } }
    }
}

impl LeftAntiContraction<Plane> for Flector {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Flector {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Point> for Flector {
    type Output = Motor;

    fn left_anti_contraction(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from(-1.0), g1: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Flector {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for Flector {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for Flector {
    type Output = Flector;

    fn left_anti_contraction(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g1: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Translator> for Flector {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group1()[0] * other.group0()[0] + self.group1()[1] * other.group0()[1] + self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Flector> for Line {
    type Output = Plane;

    fn left_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Line> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Magnitude> for Line {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl LeftAntiContraction<Motor> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: self.group0() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Line {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Point> for Line {
    type Output = Plane;

    fn left_anti_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Line {
    type Output = Horizon;

    fn left_anti_contraction(self, other: PointAtInfinity) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Rotor> for Line {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Scalar> for Line {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Flector> for LineAtOrigin {
    type Output = Plane;

    fn left_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Magnitude> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl LeftAntiContraction<Motor> for LineAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: self.group0() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftAntiContraction<Point> for LineAtOrigin {
    type Output = Plane;

    fn left_anti_contraction(self, other: Point) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for LineAtOrigin {
    type Output = Horizon;

    fn left_anti_contraction(self, other: PointAtInfinity) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Rotor> for LineAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Scalar> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftAntiContraction<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Magnitude {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * other.group1() } }
    }
}

impl LeftAntiContraction<Horizon> for Magnitude {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for Magnitude {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn left_anti_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<Motor> for Magnitude {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl LeftAntiContraction<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * other.group1(), g2: Simd32x3::from(self.group0()[1]) * other.group2(), g3: Simd32x3::from(self.group0()[1]) * other.group3(), g4: Simd32x4::from(self.group0()[1]) * other.group4() } }
    }
}

impl LeftAntiContraction<Origin> for Magnitude {
    type Output = Origin;

    fn left_anti_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Plane> for Magnitude {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Magnitude {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<Rotor> for Magnitude {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<Scalar> for Magnitude {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Translator> for Magnitude {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for Motor {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Horizon> for Motor {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for Motor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Motor {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Motor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for Motor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftAntiContraction<Motor> for Motor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[3]) * other.group0() + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group4() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Plane> for Motor {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Motor {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Motor {
    type Output = Flector;

    fn left_anti_contraction(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for Motor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for Motor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[3], self.group0()[0]]) * Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftAntiContraction<Translator> for Motor {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for MultiVector {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0(), g2: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Horizon> for MultiVector {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * other.group0(), g3: Simd32x3::from(self.group0()[1]) * other.group1(), g4: Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * other.group0(), g4: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0(), g1: swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: Simd32x3::from(0.0), g3: self.group2() * Simd32x3::from(other.group0()[0]), g4: swizzle!(self.group1(), 0, 0, 0, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + self.group0() * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[1]) * other.group1(), g4: Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * other.group1() + swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: Simd32x3::from(self.group0()[1]) * other.group2() + Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + self.group2() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group0()[1]) * other.group4() + Simd32x4::from(self.group2()[0]) * swizzle!(other.group1(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group3()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + swizzle!(self.group1(), 0, 0, 0, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from(other.group0()) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0(), g2: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group2()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group4()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group4()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + self.group0() * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group4()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group4()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: swizzle!(self.group0(), 1, 0) * Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]), g1: swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: Simd32x3::from(0.0), g3: self.group2() * Simd32x3::from(other.group0()), g4: swizzle!(self.group1(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Flector> for Origin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Magnitude> for Origin {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0() * other.group0()[0] } }
    }
}

impl LeftAntiContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Origin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Origin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Scalar> for Origin {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Plane {
    type Output = Motor;

    fn left_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Line> for Plane {
    type Output = Plane;

    fn left_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Plane {
    type Output = Horizon;

    fn left_anti_contraction(self, other: LineAtInfinity) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for Plane {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Motor> for Plane {
    type Output = Plane;

    fn left_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group3()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Plane {
    type Output = LineAtOrigin;

    fn left_anti_contraction(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Plane> for Plane {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Point> for Plane {
    type Output = Line;

    fn left_anti_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Plane {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for Plane {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for Plane {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Translator> for Plane {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Flector> for PlaneAtOrigin {
    type Output = Motor;

    fn left_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[1]]) * Simd32x4::from([0.0, -1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[2]]) * Simd32x4::from([0.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[0], other.group1()[0]]) * Simd32x4::from([-1.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Line> for PlaneAtOrigin {
    type Output = Plane;

    fn left_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Horizon;

    fn left_anti_contraction(self, other: LineAtInfinity) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Motor> for PlaneAtOrigin {
    type Output = Plane;

    fn left_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group1()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]), g2: self.group0() * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group2()[2], other.group2()[0], other.group3()[1]]) * Simd32x4::from([1.0, 0.0, -1.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group2()[1], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn left_anti_contraction(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Point> for PlaneAtOrigin {
    type Output = Line;

    fn left_anti_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftAntiContraction<Translator> for PlaneAtOrigin {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftAntiContraction<Flector> for Point {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Magnitude> for Point {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[3] * other.group0()[0] } }
    }
}

impl LeftAntiContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Point {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Point {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftAntiContraction<Scalar> for Point {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for Rotor {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Horizon> for Rotor {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for Rotor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Line) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Rotor {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Magnitude> for Rotor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftAntiContraction<Motor> for Rotor {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[3]) * other.group0() + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group4() + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Origin> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftAntiContraction<Plane> for Rotor {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn left_anti_contraction(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftAntiContraction<Scalar> for Rotor {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[3], self.group0()[0]]) * Simd32x2::from(other.group0()) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftAntiContraction<Translator> for Rotor {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<AntiScalar> for Translator {
    type Output = AntiScalar;

    fn left_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Flector> for Translator {
    type Output = Flector;

    fn left_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<Horizon> for Translator {
    type Output = Horizon;

    fn left_anti_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Line> for Translator {
    type Output = Line;

    fn left_anti_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn left_anti_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<LineAtOrigin> for Translator {
    type Output = LineAtOrigin;

    fn left_anti_contraction(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Magnitude> for Translator {
    type Output = Magnitude;

    fn left_anti_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Motor> for Translator {
    type Output = Motor;

    fn left_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl LeftAntiContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[3]) * other.group3(), g4: Simd32x4::from(self.group0()[3]) * other.group4() } }
    }
}

impl LeftAntiContraction<Origin> for Translator {
    type Output = Origin;

    fn left_anti_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Plane> for Translator {
    type Output = Plane;

    fn left_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<PlaneAtOrigin> for Translator {
    type Output = PlaneAtOrigin;

    fn left_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Point> for Translator {
    type Output = Point;

    fn left_anti_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn left_anti_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Rotor> for Translator {
    type Output = Rotor;

    fn left_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftAntiContraction<Scalar> for Translator {
    type Output = Scalar;

    fn left_anti_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftAntiContraction<Translator> for Translator {
    type Output = Translator;

    fn left_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl LeftContraction<AntiScalar> for Flector {
    type Output = Flector;

    fn left_contraction(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Flector> for Flector {
    type Output = MultiVector;

    fn left_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from(other.group1()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Horizon> for Flector {
    type Output = MultiVector;

    fn left_contraction(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[3], self.group1()[0]]) * Simd32x2::from(other.group0()) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Line> for Flector {
    type Output = Point;

    fn left_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtInfinity> for Flector {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtOrigin> for Flector {
    type Output = Origin;

    fn left_contraction(self, other: LineAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for Flector {
    type Output = Flector;

    fn left_contraction(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 0, 0, 3) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for Flector {
    type Output = Flector;

    fn left_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from(other.group4()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group3()[2], other.group3()[2], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group3()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group3()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group4()[3]) * Simd32x3::from(-1.0), g4: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for Flector {
    type Output = MultiVector;

    fn left_contraction(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[3], self.group1()[0]]) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Flector {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Flector {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<PointAtInfinity> for Flector {
    type Output = Scalar;

    fn left_contraction(self, other: PointAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for Flector {
    type Output = Flector;

    fn left_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for Flector {
    type Output = Flector;

    fn left_contraction(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[3]]) * swizzle!(other.group0(), 0, 2, 1, 3) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<AntiScalar> for Horizon {
    type Output = Origin;

    fn left_contraction(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Flector> for Horizon {
    type Output = Scalar;

    fn left_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group1()[3] } }
    }
}

impl LeftContraction<Horizon> for Horizon {
    type Output = Scalar;

    fn left_contraction(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl LeftContraction<Magnitude> for Horizon {
    type Output = Origin;

    fn left_contraction(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl LeftContraction<Motor> for Horizon {
    type Output = Origin;

    fn left_contraction(self, other: Motor) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl LeftContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], other.group4()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for Horizon {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl LeftContraction<Rotor> for Horizon {
    type Output = Origin;

    fn left_contraction(self, other: Rotor) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl LeftContraction<Translator> for Horizon {
    type Output = Origin;

    fn left_contraction(self, other: Translator) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl LeftContraction<AntiScalar> for Line {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for Line {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Horizon> for Line {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Line> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<LineAtInfinity> for Line {
    type Output = Scalar;

    fn left_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for Line {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for Line {
    type Output = MultiVector;

    fn left_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group4(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group4(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group4(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: self.group1() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for Line {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Line {
    type Output = Origin;

    fn left_contraction(self, other: PlaneAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for Line {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftContraction<Translator> for Line {
    type Output = MultiVector;

    fn left_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<AntiScalar> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for LineAtInfinity {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Horizon> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<LineAtInfinity> for LineAtInfinity {
    type Output = Scalar;

    fn left_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn left_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: self.group0() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for LineAtInfinity {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for LineAtInfinity {
    type Output = Origin;

    fn left_contraction(self, other: PlaneAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftContraction<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn left_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn left_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Flector> for Magnitude {
    type Output = Flector;

    fn left_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<Horizon> for Magnitude {
    type Output = Horizon;

    fn left_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Line> for Magnitude {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn left_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn left_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Motor> for Magnitude {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * other.group1(), g2: Simd32x3::from(self.group0()[0]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * other.group4() } }
    }
}

impl LeftContraction<Origin> for Magnitude {
    type Output = Origin;

    fn left_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Plane> for Magnitude {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Point> for Magnitude {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for Magnitude {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<Scalar> for Magnitude {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Translator> for Magnitude {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<AntiScalar> for Motor {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for Motor {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Horizon> for Motor {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Line> for Motor {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<LineAtInfinity> for Motor {
    type Output = Scalar;

    fn left_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for Motor {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group4(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group4(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group4(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: self.group1() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for Motor {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Motor {
    type Output = Origin;

    fn left_contraction(self, other: PlaneAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for Motor {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftContraction<Translator> for Motor {
    type Output = MultiVector;

    fn left_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()) * Simd32x2::from([0.0, 1.0]), g1: swizzle!(self.group4(), 0, 0, 0, 3) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: self.group3() * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from(other.group1()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group3()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(self.group0()[0]) * other.group1() } }
    }
}

impl LeftContraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group4()[3], self.group4()[0]]) * Simd32x2::from(other.group0()) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from(other.group0()) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl LeftContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[0]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()[0]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0(), g1: swizzle!(self.group4(), 0, 0, 0, 3) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: self.group3() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group3() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * other.group1(), g4: swizzle!(self.group1(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from(other.group4()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group3()[2], other.group3()[2], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group3()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[0]) * swizzle!(other.group4(), 3, 3, 3, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group4(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group4(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group3()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from(self.group0()[0]) * other.group3() + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group4()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(self.group0()[0]) * other.group4() + swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Origin> for MultiVector {
    type Output = Origin;

    fn left_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group4()[3], self.group4()[0]]) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group3()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl LeftContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * other.group0(), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group3() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: swizzle!(self.group1(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Scalar> for MultiVector {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl LeftContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn left_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group1()[0], self.group1()[0], self.group4()[3]]) * swizzle!(other.group0(), 0, 2, 1, 3) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g2: self.group3() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: swizzle!(self.group1(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<AntiScalar> for Plane {
    type Output = Origin;

    fn left_contraction(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl LeftContraction<Flector> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group1()[3] } }
    }
}

impl LeftContraction<Horizon> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl LeftContraction<Magnitude> for Plane {
    type Output = Origin;

    fn left_contraction(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0()[1] } }
    }
}

impl LeftContraction<Motor> for Plane {
    type Output = Origin;

    fn left_contraction(self, other: Motor) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[3], self.group0()[0]]) * Simd32x2::from([other.group4()[3], other.group4()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for Plane {
    type Output = Scalar;

    fn left_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Rotor> for Plane {
    type Output = Origin;

    fn left_contraction(self, other: Rotor) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<Translator> for Plane {
    type Output = Origin;

    fn left_contraction(self, other: Translator) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0()[3] } }
    }
}

impl LeftContraction<AntiScalar> for Point {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for Point {
    type Output = MultiVector;

    fn left_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Horizon> for Point {
    type Output = LineAtInfinity;

    fn left_contraction(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftContraction<Line> for Point {
    type Output = Point;

    fn left_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtOrigin> for Point {
    type Output = Origin;

    fn left_contraction(self, other: LineAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for Point {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for Point {
    type Output = Flector;

    fn left_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group3()[2], other.group3()[2], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group3()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group3()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group4()[3]) * Simd32x3::from(-1.0), g4: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for Point {
    type Output = Line;

    fn left_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Point {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<PointAtInfinity> for Point {
    type Output = Scalar;

    fn left_contraction(self, other: PointAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for Point {
    type Output = Flector;

    fn left_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for Point {
    type Output = Flector;

    fn left_contraction(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<AntiScalar> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn left_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: self.group0() * Simd32x3::from(other.group1()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Horizon> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn left_contraction(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) * Simd32x3::from(-1.0) } }
    }
}

impl LeftContraction<Line> for PointAtInfinity {
    type Output = Point;

    fn left_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtInfinity> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl LeftContraction<LineAtOrigin> for PointAtInfinity {
    type Output = Origin;

    fn left_contraction(self, other: LineAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for PointAtInfinity {
    type Output = Flector;

    fn left_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group1()[2], other.group1()[0], other.group0()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group1()[1], other.group0()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group3()[2], other.group3()[2], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group3()[1], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group3()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g3: self.group0() * Simd32x3::from(other.group4()[3]) * Simd32x3::from(-1.0), g4: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Plane> for PointAtInfinity {
    type Output = Line;

    fn left_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]), g1: self.group0() * Simd32x3::from(other.group0()[3]) * Simd32x3::from(-1.0) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl LeftContraction<Point> for PointAtInfinity {
    type Output = Scalar;

    fn left_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<PointAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn left_contraction(self, other: PointAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn left_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<Translator> for PointAtInfinity {
    type Output = Flector;

    fn left_contraction(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl LeftContraction<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn left_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Flector> for Scalar {
    type Output = Flector;

    fn left_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<Horizon> for Scalar {
    type Output = Horizon;

    fn left_contraction(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Line> for Scalar {
    type Output = Line;

    fn left_contraction(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn left_contraction(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Magnitude> for Scalar {
    type Output = Magnitude;

    fn left_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Motor> for Scalar {
    type Output = Motor;

    fn left_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl LeftContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl LeftContraction<Origin> for Scalar {
    type Output = Origin;

    fn left_contraction(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Plane> for Scalar {
    type Output = Plane;

    fn left_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn left_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Point> for Scalar {
    type Output = Point;

    fn left_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Rotor> for Scalar {
    type Output = Rotor;

    fn left_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn left_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl LeftContraction<Translator> for Scalar {
    type Output = Translator;

    fn left_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl LeftContraction<AntiScalar> for Translator {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Flector> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<Horizon> for Translator {
    type Output = PointAtInfinity;

    fn left_contraction(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl LeftContraction<Line> for Translator {
    type Output = Scalar;

    fn left_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl LeftContraction<LineAtInfinity> for Translator {
    type Output = Scalar;

    fn left_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Magnitude> for Translator {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]) } }
    }
}

impl LeftContraction<Motor> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl LeftContraction<Plane> for Translator {
    type Output = Point;

    fn left_contraction(self, other: Plane) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl LeftContraction<PlaneAtOrigin> for Translator {
    type Output = Origin;

    fn left_contraction(self, other: PlaneAtOrigin) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl LeftContraction<Rotor> for Translator {
    type Output = LineAtOrigin;

    fn left_contraction(self, other: Rotor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl LeftContraction<Translator> for Translator {
    type Output = MultiVector;

    fn left_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightAntiContraction<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Magnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Motor> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Motor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<MultiVector> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: MultiVector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Rotor> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Rotor) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<Translator> for AntiScalar {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Translator) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<AntiScalar> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Flector {
    type Output = Motor;

    fn right_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Line> for Flector {
    type Output = Plane;

    fn right_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Flector {
    type Output = Plane;

    fn right_anti_contraction(self, other: LineAtOrigin) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Magnitude> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: self.group1() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[1]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Origin> for Flector {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Flector {
    type Output = Motor;

    fn right_anti_contraction(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Flector {
    type Output = Motor;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]), g1: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Point> for Flector {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightAntiContraction<Rotor> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for Flector {
    type Output = Flector;

    fn right_anti_contraction(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: AntiScalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Magnitude> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Motor> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Motor) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<MultiVector> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: MultiVector) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Rotor> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Rotor) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<Translator> for Horizon {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<AntiScalar> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Line {
    type Output = Plane;

    fn right_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Line> for Line {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Line {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Line {
    type Output = Motor;

    fn right_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[1]), g3: self.group1() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group4(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Plane> for Line {
    type Output = Plane;

    fn right_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Line {
    type Output = Plane;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group1()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Line {
    type Output = Motor;

    fn right_anti_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for Line {
    type Output = Line;

    fn right_anti_contraction(self, other: Translator) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: AntiScalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for LineAtInfinity {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Flector) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Motor) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: self.group0() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Plane> for LineAtInfinity {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Plane) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for LineAtInfinity {
    type Output = Horizon;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Rotor> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Rotor) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Translator) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn right_anti_contraction(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Flector) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for LineAtOrigin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<LineAtOrigin> for LineAtOrigin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn right_anti_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for LineAtOrigin {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Plane) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn right_anti_contraction(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn right_anti_contraction(self, other: AntiScalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Magnitude {
    type Output = Flector;

    fn right_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for Magnitude {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Line) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Magnitude {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: LineAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightAntiContraction<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn right_anti_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Magnitude {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightAntiContraction<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[1]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * other.group2(), g4: Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Origin> for Magnitude {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Origin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Magnitude {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Magnitude {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl RightAntiContraction<Point> for Magnitude {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Point) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[0] * other.group0()[3] } }
    }
}

impl RightAntiContraction<Rotor> for Magnitude {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightAntiContraction<Translator> for Magnitude {
    type Output = Magnitude;

    fn right_anti_contraction(self, other: Translator) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Motor {
    type Output = Plane;

    fn right_anti_contraction(self, other: Flector) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Line> for Motor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Motor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: self.group1() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Plane> for Motor {
    type Output = Plane;

    fn right_anti_contraction(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Motor {
    type Output = Plane;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Rotor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for Motor {
    type Output = Motor;

    fn right_anti_contraction(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: self.group1() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group2()[0]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * other.group0(), g4: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * other.group0(), g4: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[1]), g1: self.group1() * Simd32x4::from(other.group0()[1]), g2: self.group2() * Simd32x3::from(other.group0()[1]), g3: self.group3() * Simd32x3::from(other.group0()[1]), g4: self.group4() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + self.group0() * Simd32x2::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]), g2: self.group2() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group3() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from(other.group1()[3]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + self.group0() * Simd32x2::from(other.group0()[1]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) + self.group2() * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([1.0, 0.0, 0.0]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 1.0, 0.0]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from(other.group0()[1]) * Simd32x3::from([0.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[0]) * swizzle!(other.group4(), 2, 2, 1, 2) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) + Simd32x4::from(self.group2()[1]) * swizzle!(other.group4(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group4(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from(other.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Origin> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from(other.group0()) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group2()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group2()[0], self.group2()[0], self.group2()[0], self.group3()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0(), g3: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from([self.group2()[0], self.group2()[0], self.group2()[0], self.group3()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, -1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group1()[0], self.group1()[3]]) * Simd32x2::from([other.group0()[0], other.group0()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([0.0, -1.0]) + self.group0() * Simd32x2::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]), g2: self.group2() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group3() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: self.group1() * Simd32x4::from(other.group0()[3]), g2: self.group2() * Simd32x3::from(other.group0()[3]), g3: self.group3() * Simd32x3::from(other.group0()[3]), g4: self.group4() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Origin {
    type Output = Origin;

    fn right_anti_contraction(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Flector> for Origin {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Flector) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) } }
    }
}

impl RightAntiContraction<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Line) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Magnitude> for Origin {
    type Output = Origin;

    fn right_anti_contraction(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Motor> for Origin {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for Origin {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Origin> for Origin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Origin {
    type Output = LineAtOrigin;

    fn right_anti_contraction(self, other: Plane) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Point> for Origin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<Rotor> for Origin {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Translator> for Origin {
    type Output = Origin;

    fn right_anti_contraction(self, other: Translator) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<AntiScalar> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Plane {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Magnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Motor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Plane> for Plane {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Plane {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Rotor> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Rotor) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for Plane {
    type Output = Plane;

    fn right_anti_contraction(self, other: Translator) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Flector) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group1()[0] + self.group0()[1] * other.group1()[1] + self.group0()[2] * other.group1()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Motor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group4()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group4()[2]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group4()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Plane> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Rotor> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Rotor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Translator) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Point {
    type Output = Motor;

    fn right_anti_contraction(self, other: Flector) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Line> for Point {
    type Output = Plane;

    fn right_anti_contraction(self, other: Line) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Point {
    type Output = Plane;

    fn right_anti_contraction(self, other: LineAtOrigin) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Magnitude> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: Magnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Point {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from([other.group1()[0], other.group1()[3]]) * Simd32x2::from([0.0, -1.0]), g1: self.group0() * Simd32x4::from(other.group0()[1]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group2()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Origin> for Point {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Point {
    type Output = Line;

    fn right_anti_contraction(self, other: Plane) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g1: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Point {
    type Output = Line;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(), g1: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Point> for Point {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Point) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightAntiContraction<Rotor> for Point {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for Point {
    type Output = Point;

    fn right_anti_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: AntiScalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Flector) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Line> for PointAtInfinity {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Line) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<LineAtOrigin> for PointAtInfinity {
    type Output = Horizon;

    fn right_anti_contraction(self, other: LineAtOrigin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for PointAtInfinity {
    type Output = Flector;

    fn right_anti_contraction(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], other.group4()[2], other.group4()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group4()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[0], other.group4()[2], other.group4()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Plane> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Plane) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn right_anti_contraction(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Rotor {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Flector) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for Rotor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: Line) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Rotor {
    type Output = AntiScalar;

    fn right_anti_contraction(self, other: LineAtOrigin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Magnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group2()[1]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group2()[2]) * Simd32x2::from([0.0, -1.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group2()[0]) * Simd32x2::from([0.0, -1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[1]) * swizzle!(other.group4(), 2, 2, 0, 2) * Simd32x4::from([1.0, 0.0, -1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group4(), 1, 0, 1, 1) * Simd32x4::from([-1.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group4(), 0, 2, 1, 0) * Simd32x4::from([0.0, -1.0, 1.0, 0.0]) } }
    }
}

impl RightAntiContraction<Plane> for Rotor {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: Plane) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Rotor {
    type Output = PlaneAtOrigin;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([1.0, 0.0, -1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([-1.0, 1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, -1.0, 1.0]) } }
    }
}

impl RightAntiContraction<Rotor> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([0.0, 1.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([0.0, 0.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 0, 0, 0) * Simd32x4::from([1.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Translator> for Rotor {
    type Output = Rotor;

    fn right_anti_contraction(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<AntiScalar> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Flector> for Scalar {
    type Output = Flector;

    fn right_anti_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Line> for Scalar {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: Line) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<LineAtOrigin> for Scalar {
    type Output = LineAtInfinity;

    fn right_anti_contraction(self, other: LineAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Magnitude> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: Magnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl RightAntiContraction<Motor> for Scalar {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightAntiContraction<MultiVector> for Scalar {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * swizzle!(other.group0(), 1, 0) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group4(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * other.group2(), g4: Simd32x4::from(self.group0()) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightAntiContraction<Origin> for Scalar {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Origin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightAntiContraction<Plane> for Scalar {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Scalar {
    type Output = PointAtInfinity;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightAntiContraction<Point> for Scalar {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Point) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<Rotor> for Scalar {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightAntiContraction<Translator> for Scalar {
    type Output = Scalar;

    fn right_anti_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl RightAntiContraction<AntiScalar> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightAntiContraction<Flector> for Translator {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Flector) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightAntiContraction<Magnitude> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Magnitude) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl RightAntiContraction<Motor> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Motor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn right_anti_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[3]]) * other.group0() * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) } }
    }
}

impl RightAntiContraction<Plane> for Translator {
    type Output = Horizon;

    fn right_anti_contraction(self, other: Plane) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<PlaneAtOrigin> for Translator {
    type Output = Horizon;

    fn right_anti_contraction(self, other: PlaneAtOrigin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightAntiContraction<Rotor> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Rotor) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightAntiContraction<Translator> for Translator {
    type Output = Translator;

    fn right_anti_contraction(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[3]) } }
    }
}

impl RightContraction<Flector> for AntiScalar {
    type Output = Flector;

    fn right_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for AntiScalar {
    type Output = Origin;

    fn right_contraction(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl RightContraction<Line> for AntiScalar {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Line) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RightContraction<LineAtInfinity> for AntiScalar {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: LineAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn right_contraction(self, other: Magnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<Motor> for AntiScalar {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Motor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group0()) * swizzle!(other.group4(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for AntiScalar {
    type Output = Origin;

    fn right_contraction(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl RightContraction<Point> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: Point) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<PointAtInfinity> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: PointAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn right_contraction(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Translator> for AntiScalar {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<Flector> for Flector {
    type Output = MultiVector;

    fn right_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from(other.group1()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Horizon> for Flector {
    type Output = Scalar;

    fn right_contraction(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[3] * other.group0() } }
    }
}

impl RightContraction<Line> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<LineAtInfinity> for Flector {
    type Output = Point;

    fn right_contraction(self, other: LineAtInfinity) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Magnitude> for Flector {
    type Output = Flector;

    fn right_contraction(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<MultiVector> for Flector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from(other.group4()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + self.group0() * Simd32x4::from(other.group0()[0]), g2: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Plane> for Flector {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Flector {
    type Output = MultiVector;

    fn right_contraction(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn right_contraction(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group1()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Scalar> for Flector {
    type Output = Flector;

    fn right_contraction(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Flector {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group1(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Flector> for Horizon {
    type Output = MultiVector;

    fn right_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[3], other.group1()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Horizon> for Horizon {
    type Output = Scalar;

    fn right_contraction(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl RightContraction<Line> for Horizon {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Line) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RightContraction<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Magnitude> for Horizon {
    type Output = Horizon;

    fn right_contraction(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<Motor> for Horizon {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Motor) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for Horizon {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], other.group4()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) } }
    }
}

impl RightContraction<Plane> for Horizon {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Horizon {
    type Output = LineAtInfinity;

    fn right_contraction(self, other: Point) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn right_contraction(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl RightContraction<Scalar> for Horizon {
    type Output = Horizon;

    fn right_contraction(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Translator> for Horizon {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<Flector> for Line {
    type Output = Point;

    fn right_contraction(self, other: Flector) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Line> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RightContraction<LineAtInfinity> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for Line {
    type Output = Line;

    fn right_contraction(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group1()[0] - self.group1()[1] * other.group1()[1] - self.group1()[2] * other.group1()[2] } }
    }
}

impl RightContraction<MultiVector> for Line {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g2: self.group0() * Simd32x3::from(other.group0()[0]), g3: self.group1() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for Line {
    type Output = Point;

    fn right_contraction(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for Line {
    type Output = Point;

    fn right_contraction(self, other: PointAtInfinity) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group1()[0], self.group1()[0], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]) } }
    }
}

impl RightContraction<Scalar> for Line {
    type Output = Line;

    fn right_contraction(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Line {
    type Output = Scalar;

    fn right_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group1()[0] * other.group0()[0] - self.group1()[1] * other.group0()[1] - self.group1()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Flector> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Flector) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Line> for LineAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: Line) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightContraction<LineAtInfinity> for LineAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: LineAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_contraction(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for LineAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: Motor) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightContraction<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group1(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g2: Simd32x3::from(0.0), g3: self.group0() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Point) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn right_contraction(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for LineAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Flector> for LineAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: Flector) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: self.group0() * Simd32x3::from(other.group0()[0]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for LineAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: Point) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<PointAtInfinity> for LineAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: PointAtInfinity) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Scalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Flector> for Magnitude {
    type Output = Flector;

    fn right_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g1: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for Magnitude {
    type Output = Origin;

    fn right_contraction(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[1] * other.group0() } }
    }
}

impl RightContraction<Line> for Magnitude {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Line) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl RightContraction<LineAtInfinity> for Magnitude {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: LineAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl RightContraction<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn right_contraction(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Magnitude {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Motor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group4(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for Magnitude {
    type Output = Origin;

    fn right_contraction(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[1] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Magnitude {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: Point) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<PointAtInfinity> for Magnitude {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: PointAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl RightContraction<Scalar> for Magnitude {
    type Output = Magnitude;

    fn right_contraction(self, other: Scalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Magnitude {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<Flector> for Motor {
    type Output = Flector;

    fn right_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for Motor {
    type Output = Origin;

    fn right_contraction(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightContraction<Line> for Motor {
    type Output = MultiVector;

    fn right_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn right_contraction(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Magnitude> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Motor {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<MultiVector> for Motor {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g3: self.group1() * Simd32x3::from(other.group0()[0]), g4: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for Motor {
    type Output = Origin;

    fn right_contraction(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Motor {
    type Output = Flector;

    fn right_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for Motor {
    type Output = Flector;

    fn right_contraction(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[1], other.group0()[2]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Scalar> for Motor {
    type Output = Motor;

    fn right_contraction(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Motor {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Flector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from(other.group1()[3]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * swizzle!(other.group0(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group1(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group4()[3], self.group4()[0]]) * Simd32x2::from(other.group0()) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * Simd32x4::from(other.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Line> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group4(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group4(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[0]), g1: self.group1() * Simd32x4::from(other.group0()[0]), g2: self.group2() * Simd32x3::from(other.group0()[0]), g3: self.group3() * Simd32x3::from(other.group0()[0]), g4: self.group4() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group4(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from(other.group4()[3]) * Simd32x2::from([-1.0, 0.0]) + self.group0() * Simd32x2::from(other.group0()[0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[0]) * swizzle!(other.group1(), 2, 2, 1, 2) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) + Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group4(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + self.group2() * Simd32x3::from(other.group0()[0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group3() * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(self.group4()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 0.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 1.0, 0.0, 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group4()[3], self.group4()[0]]) * Simd32x2::from([other.group0()[3], other.group0()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group0()[1]]) * swizzle!(other.group0(), 0, 0, 0, 3) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group2()[0], self.group3()[0], self.group3()[0], self.group2()[0]]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g2: Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from(self.group2()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from([self.group2()[0], self.group3()[0], self.group3()[0], self.group2()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 1.0]), g2: Simd32x3::from(self.group4()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group4()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group4()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group4()[3]) * other.group0(), g4: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Scalar> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for MultiVector {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group4()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group4(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Magnitude> for Origin {
    type Output = Origin;

    fn right_contraction(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<MultiVector> for Origin {
    type Output = Origin;

    fn right_contraction(self, other: MultiVector) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<Scalar> for Origin {
    type Output = Origin;

    fn right_contraction(self, other: Scalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Flector> for Plane {
    type Output = MultiVector;

    fn right_contraction(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[3], self.group0()[0]]) * Simd32x2::from([other.group1()[3], other.group1()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Horizon> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightContraction<Line> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Line) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<LineAtInfinity> for Plane {
    type Output = Point;

    fn right_contraction(self, other: LineAtInfinity) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Magnitude> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Magnitude) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Motor) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group1()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<MultiVector> for Plane {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[3], self.group0()[0]]) * Simd32x2::from([other.group4()[3], other.group4()[0]]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group3()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Plane> for Plane {
    type Output = Scalar;

    fn right_contraction(self, other: Plane) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Plane {
    type Output = Line;

    fn right_contraction(self, other: Point) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<PointAtInfinity> for Plane {
    type Output = Line;

    fn right_contraction(self, other: PointAtInfinity) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightContraction<Scalar> for Plane {
    type Output = Plane;

    fn right_contraction(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Plane {
    type Output = Point;

    fn right_contraction(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Flector> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Flector) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Line> for PlaneAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: Line) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightContraction<LineAtInfinity> for PlaneAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: LineAtInfinity) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for PlaneAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: Motor) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group1()[0] - self.group0()[1] * other.group1()[1] - self.group0()[2] * other.group1()[2] } }
    }
}

impl RightContraction<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[2], other.group1()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[2], other.group1()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]) } }
    }
}

impl RightContraction<Point> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Point) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[2], other.group0()[0]]) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], other.group0()[0], other.group0()[1]]) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[2], other.group0()[1]]) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: PointAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0) * Simd32x3::from([-1.0, 0.0, 1.0]) + Simd32x3::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1) * Simd32x3::from([1.0, -1.0, 0.0]) + Simd32x3::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1) * Simd32x3::from([0.0, 1.0, -1.0]) } }
    }
}

impl RightContraction<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn right_contraction(self, other: Scalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for PlaneAtOrigin {
    type Output = Origin;

    fn right_contraction(self, other: Translator) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[0] * other.group0()[0] - self.group0()[1] * other.group0()[1] - self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Flector> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for Point {
    type Output = Point;

    fn right_contraction(self, other: Magnitude) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<MultiVector> for Point {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: self.group0() * Simd32x4::from(other.group0()[0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<PointAtInfinity> for Point {
    type Output = Scalar;

    fn right_contraction(self, other: PointAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Scalar> for Point {
    type Output = Point;

    fn right_contraction(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Flector> for PointAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: Flector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl RightContraction<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([1.0, 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Point> for PointAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<PointAtInfinity> for PointAtInfinity {
    type Output = Scalar;

    fn right_contraction(self, other: PointAtInfinity) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[0] * other.group0()[0] + self.group0()[1] * other.group0()[1] + self.group0()[2] * other.group0()[2] } }
    }
}

impl RightContraction<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn right_contraction(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl RightContraction<Flector> for Rotor {
    type Output = Flector;

    fn right_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for Rotor {
    type Output = Origin;

    fn right_contraction(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightContraction<Line> for Rotor {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Line) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightContraction<LineAtInfinity> for Rotor {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: LineAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl RightContraction<Magnitude> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Magnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Rotor {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Motor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group1() } }
    }
}

impl RightContraction<MultiVector> for Rotor {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from([self.group0()[0], self.group0()[3]]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from(other.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g2: Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g3: Simd32x3::from(0.0), g4: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for Rotor {
    type Output = Origin;

    fn right_contraction(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Rotor {
    type Output = Flector;

    fn right_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn right_contraction(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from(other.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from(other.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from(other.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Scalar> for Rotor {
    type Output = Rotor;

    fn right_contraction(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Rotor {
    type Output = LineAtOrigin;

    fn right_contraction(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl RightContraction<Magnitude> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Magnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<MultiVector> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: MultiVector) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl RightContraction<Scalar> for Scalar {
    type Output = Scalar;

    fn right_contraction(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl RightContraction<Flector> for Translator {
    type Output = Flector;

    fn right_contraction(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group1()[3]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Horizon> for Translator {
    type Output = Origin;

    fn right_contraction(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl RightContraction<Line> for Translator {
    type Output = MultiVector;

    fn right_contraction(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn right_contraction(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<Magnitude> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Magnitude) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl RightContraction<Motor> for Translator {
    type Output = MultiVector;

    fn right_contraction(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group1()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group1()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group1()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl RightContraction<MultiVector> for Translator {
    type Output = MultiVector;

    fn right_contraction(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group3()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group3()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([0.0, 1.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group3()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(self.group0()[1]) * swizzle!(other.group1(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group1(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + swizzle!(self.group0(), 0, 0, 0, 3) * Simd32x4::from([other.group1()[0], other.group1()[2], other.group1()[1], other.group4()[3]]) * Simd32x4::from([0.0, 1.0, -1.0, -1.0]), g2: Simd32x3::from(self.group0()[3]) * other.group3(), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Plane> for Translator {
    type Output = Origin;

    fn right_contraction(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0()[3] * other.group0()[3] } }
    }
}

impl RightContraction<Point> for Translator {
    type Output = Flector;

    fn right_contraction(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 2, 0, 2) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 1, 1) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 0, 2, 1, 0) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<PointAtInfinity> for Translator {
    type Output = Flector;

    fn right_contraction(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[2], other.group0()[0], other.group0()[2]]) * Simd32x4::from([-1.0, 0.0, 1.0, 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], other.group0()[0], other.group0()[1], other.group0()[1]]) * Simd32x4::from([1.0, -1.0, 0.0, 0.0]) + Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[2], other.group0()[1], other.group0()[0]]) * Simd32x4::from([0.0, 1.0, -1.0, 0.0]), g1: swizzle!(self.group0(), 3, 3, 3, 0) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]) } }
    }
}

impl RightContraction<Scalar> for Translator {
    type Output = Translator;

    fn right_contraction(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl RightContraction<Translator> for Translator {
    type Output = MultiVector;

    fn right_contraction(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from(other.group0()[1]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from(other.group0()[2]) * Simd32x2::from([-1.0, 0.0]) + Simd32x2::from(self.group0()[0]) * Simd32x2::from(other.group0()[0]) * Simd32x2::from([-1.0, 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

