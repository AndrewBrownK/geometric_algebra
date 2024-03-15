
#![allow(clippy::assign_op_pattern)]
use crate::rga3d::*;


/// Geometric Product
/// Synonyms included: GeometricProduct, WedgeDot
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait GeometricProduct<T> {
    type Output;
    fn geometric_product(self, other: T) -> Self::Output;
}


/// Geometric Product
/// Synonyms included: GeometricProduct, WedgeDot
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait WedgeDot<T> {
    type Output;
    fn wedge_dot(self, other: T) -> Self::Output;
}


/// Geometric Anti-Product
/// Synonyms included: GeometricAntiProduct, AntiWedgeDot
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait GeometricAntiProduct<T> {
    type Output;
    fn geometric_anti_product(self, other: T) -> Self::Output;
}


/// Geometric Anti-Product
/// Synonyms included: GeometricAntiProduct, AntiWedgeDot
/// https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
pub trait AntiWedgeDot<T> {
    type Output;
    fn anti_wedge_dot(self, other: T) -> Self::Output;
}

impl AntiWedgeDot<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn anti_wedge_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Flector> for AntiScalar {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl AntiWedgeDot<Horizon> for AntiScalar {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Line> for AntiScalar {
    type Output = Line;

    fn anti_wedge_dot(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn anti_wedge_dot(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Motor> for AntiScalar {
    type Output = Motor;

    fn anti_wedge_dot(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl AntiWedgeDot<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl AntiWedgeDot<Origin> for AntiScalar {
    type Output = Origin;

    fn anti_wedge_dot(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Plane> for AntiScalar {
    type Output = Plane;

    fn anti_wedge_dot(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Point> for AntiScalar {
    type Output = Point;

    fn anti_wedge_dot(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Rotor> for AntiScalar {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Scalar> for AntiScalar {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Translator> for AntiScalar {
    type Output = Translator;

    fn anti_wedge_dot(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<AntiScalar> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Horizon> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Line> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<Magnitude> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) + swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + self.group1() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl AntiWedgeDot<Origin> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Plane> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0()[3], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Point> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Rotor> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group1(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl AntiWedgeDot<Translator> for Flector {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Horizon {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: AntiScalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Flector> for Horizon {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Line> for Horizon {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Line) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Horizon {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Magnitude> for Horizon {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl AntiWedgeDot<Motor> for Horizon {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for Horizon {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl AntiWedgeDot<Origin> for Horizon {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Origin) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Plane) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Point> for Horizon {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl AntiWedgeDot<Rotor> for Horizon {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<Translator> for Horizon {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiWedgeDot<AntiScalar> for Line {
    type Output = Line;

    fn anti_wedge_dot(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<Horizon> for Line {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Line> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Magnitude> for Line {
    type Output = Line;

    fn anti_wedge_dot(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl AntiWedgeDot<Plane> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Line {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Rotor> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Scalar> for Line {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Translator> for Line {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + self.group1() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: AntiScalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for LineAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Origin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for LineAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for LineAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Point) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiWedgeDot<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Translator) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge_dot(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for LineAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Horizon> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Line> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for LineAtOrigin {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Magnitude> for LineAtOrigin {
    type Output = Line;

    fn anti_wedge_dot(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl AntiWedgeDot<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_wedge_dot(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for LineAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for LineAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<Point> for LineAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn anti_wedge_dot(self, other: AntiScalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Magnitude {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * other.group1() } }
    }
}

impl AntiWedgeDot<Horizon> for Magnitude {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl AntiWedgeDot<Line> for Magnitude {
    type Output = Line;

    fn anti_wedge_dot(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Magnitude {
    type Output = Line;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl AntiWedgeDot<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn anti_wedge_dot(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl AntiWedgeDot<Motor> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group1(), g2: Simd32x3::from(self.group0()[1]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * other.group4() } }
    }
}

impl AntiWedgeDot<Origin> for Magnitude {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl AntiWedgeDot<Plane> for Magnitude {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Magnitude {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<Point> for Magnitude {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl AntiWedgeDot<Rotor> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Scalar> for Magnitude {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl AntiWedgeDot<Translator> for Magnitude {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Motor {
    type Output = Motor;

    fn anti_wedge_dot(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<Horizon> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl AntiWedgeDot<Line> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Magnitude> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Motor> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * other.group0() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl AntiWedgeDot<Plane> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Motor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Rotor> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Scalar> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Translator> for Motor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group1() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<Horizon> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl AntiWedgeDot<Line> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()[1]) + swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g2: self.group2() * Simd32x3::from(other.group0()[1]), g3: self.group2() * Simd32x3::from(other.group0()[0]) + self.group3() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + self.group4() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group2()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * other.group2() - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * other.group4() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl AntiWedgeDot<Origin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl AntiWedgeDot<Plane> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group0()[3], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Rotor> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group1(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group4(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0(), 0.0]), g1: swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]), g2: Simd32x3::from(0.0), g3: self.group2() * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl AntiWedgeDot<Translator> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: self.group2() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + self.group3() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Origin {
    type Output = Origin;

    fn anti_wedge_dot(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Flector> for Origin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[3], -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Horizon> for Origin {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Line> for Origin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Magnitude> for Origin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) } }
    }
}

impl AntiWedgeDot<Motor> for Origin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<MultiVector> for Origin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], -other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) } }
    }
}

impl AntiWedgeDot<Origin> for Origin {
    type Output = AntiScalar;

    fn anti_wedge_dot(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Plane> for Origin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Point> for Origin {
    type Output = Translator;

    fn anti_wedge_dot(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Origin {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Rotor> for Origin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for Origin {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Translator> for Origin {
    type Output = Point;

    fn anti_wedge_dot(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Plane {
    type Output = Plane;

    fn anti_wedge_dot(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Plane {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Horizon> for Plane {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Line> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<Magnitude> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for Plane {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl AntiWedgeDot<Origin> for Plane {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Plane> for Plane {
    type Output = Motor;

    fn anti_wedge_dot(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Plane {
    type Output = Motor;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<Point> for Plane {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Plane {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Rotor> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for Plane {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Translator> for Plane {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn anti_wedge_dot(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Horizon> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl AntiWedgeDot<Magnitude> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl AntiWedgeDot<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn anti_wedge_dot(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for PlaneAtOrigin {
    type Output = Motor;

    fn anti_wedge_dot(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Point {
    type Output = Point;

    fn anti_wedge_dot(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Point {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Horizon> for Point {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl AntiWedgeDot<Line> for Point {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Point {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Magnitude> for Point {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) } }
    }
}

impl AntiWedgeDot<Motor> for Point {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl AntiWedgeDot<MultiVector> for Point {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) } }
    }
}

impl AntiWedgeDot<Origin> for Point {
    type Output = Translator;

    fn anti_wedge_dot(self, other: Origin) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) - self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for Point {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Point {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Point> for Point {
    type Output = Translator;

    fn anti_wedge_dot(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Point {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<Rotor> for Point {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl AntiWedgeDot<Scalar> for Point {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl AntiWedgeDot<Translator> for Point {
    type Output = Point;

    fn anti_wedge_dot(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: AntiScalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Line> for PointAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for PointAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl AntiWedgeDot<Motor> for PointAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) } }
    }
}

impl AntiWedgeDot<Origin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Origin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for PointAtInfinity {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Point> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Point) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiWedgeDot<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Rotor {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl AntiWedgeDot<Horizon> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl AntiWedgeDot<Line> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<Magnitude> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Motor> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() } }
    }
}

impl AntiWedgeDot<Origin> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl AntiWedgeDot<Plane> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl AntiWedgeDot<Point> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Rotor> for Rotor {
    type Output = Rotor;

    fn anti_wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<Scalar> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Translator> for Rotor {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<AntiScalar> for Scalar {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Flector> for Scalar {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl AntiWedgeDot<Line> for Scalar {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: Line) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Scalar {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Magnitude> for Scalar {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Magnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl AntiWedgeDot<Motor> for Scalar {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Scalar {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[1], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * other.group2(), g4: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) } }
    }
}

impl AntiWedgeDot<Origin> for Scalar {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Origin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl AntiWedgeDot<Plane> for Scalar {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Scalar {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl AntiWedgeDot<Point> for Scalar {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Point) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiWedgeDot<Rotor> for Scalar {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Translator> for Scalar {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl AntiWedgeDot<AntiScalar> for Translator {
    type Output = Translator;

    fn anti_wedge_dot(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Flector> for Translator {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl AntiWedgeDot<Horizon> for Translator {
    type Output = Horizon;

    fn anti_wedge_dot(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl AntiWedgeDot<Line> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn anti_wedge_dot(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Magnitude> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Motor> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<MultiVector> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group2()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() } }
    }
}

impl AntiWedgeDot<Origin> for Translator {
    type Output = Point;

    fn anti_wedge_dot(self, other: Origin) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl AntiWedgeDot<Plane> for Translator {
    type Output = Flector;

    fn anti_wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn anti_wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl AntiWedgeDot<Point> for Translator {
    type Output = Point;

    fn anti_wedge_dot(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn anti_wedge_dot(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl AntiWedgeDot<Rotor> for Translator {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl AntiWedgeDot<Scalar> for Translator {
    type Output = Scalar;

    fn anti_wedge_dot(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl AntiWedgeDot<Translator> for Translator {
    type Output = Translator;

    fn anti_wedge_dot(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_anti_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<Horizon> for AntiScalar {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Line> for AntiScalar {
    type Output = Line;

    fn geometric_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for AntiScalar {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_anti_product(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Magnitude> for AntiScalar {
    type Output = Magnitude;

    fn geometric_anti_product(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Motor> for AntiScalar {
    type Output = Motor;

    fn geometric_anti_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricAntiProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl GeometricAntiProduct<Origin> for AntiScalar {
    type Output = Origin;

    fn geometric_anti_product(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for AntiScalar {
    type Output = Plane;

    fn geometric_anti_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for AntiScalar {
    type Output = Point;

    fn geometric_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for AntiScalar {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for AntiScalar {
    type Output = Rotor;

    fn geometric_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Scalar> for AntiScalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for AntiScalar {
    type Output = Translator;

    fn geometric_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Horizon> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Line> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]) + swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + self.group1() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0()[3], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Point> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, -other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Rotor> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group1(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl GeometricAntiProduct<Translator> for Flector {
    type Output = Flector;

    fn geometric_anti_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Horizon {
    type Output = Horizon;

    fn geometric_anti_product(self, other: AntiScalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Flector> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Line> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Line) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: LineAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Magnitude> for Horizon {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Magnitude) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl GeometricAntiProduct<Motor> for Horizon {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Horizon {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Horizon {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Origin) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Plane) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for Horizon {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Point) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl GeometricAntiProduct<Rotor> for Horizon {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<Translator> for Horizon {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Translator) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Line {
    type Output = Line;

    fn geometric_anti_product(self, other: AntiScalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<Horizon> for Line {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Line> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Line {
    type Output = Line;

    fn geometric_anti_product(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Line {
    type Output = Flector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Scalar> for Line {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Translator> for Line {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + self.group1() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: AntiScalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for LineAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Magnitude> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Magnitude) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Origin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for LineAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for LineAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Point) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<Rotor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Translator> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Translator) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_product(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Horizon> for LineAtOrigin {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Line> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for LineAtOrigin {
    type Output = Line;

    fn geometric_anti_product(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricAntiProduct<Motor> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_product(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Point> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Rotor> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for LineAtOrigin {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Translator> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_anti_product(self, other: AntiScalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Magnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * other.group1() } }
    }
}

impl GeometricAntiProduct<Horizon> for Magnitude {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl GeometricAntiProduct<Line> for Magnitude {
    type Output = Line;

    fn geometric_anti_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Magnitude {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Magnitude {
    type Output = Line;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn geometric_anti_product(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Motor> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group1(), g2: Simd32x3::from(self.group0()[1]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * other.group4() } }
    }
}

impl GeometricAntiProduct<Origin> for Magnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl GeometricAntiProduct<Plane> for Magnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Magnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<Point> for Magnitude {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Magnitude {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Scalar> for Magnitude {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[1] * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for Magnitude {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Motor {
    type Output = Motor;

    fn geometric_anti_product(self, other: AntiScalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<Horizon> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl GeometricAntiProduct<Line> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * other.group0() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group2()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Motor {
    type Output = Flector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Scalar> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group1() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<Horizon> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl GeometricAntiProduct<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0(), g1: self.group1() * Simd32x4::from(other.group0()[1]) + swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g2: self.group2() * Simd32x3::from(other.group0()[1]), g3: self.group2() * Simd32x3::from(other.group0()[0]) + self.group3() * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + self.group4() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[1]) * other.group0() + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]) - Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group2()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]) + Simd32x4::from(self.group0()[1]) * other.group1() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(self.group0()[1]) * other.group2() - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) + Simd32x4::from(self.group0()[1]) * other.group4() + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl GeometricAntiProduct<Origin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group0()[3], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[1]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group1(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group4(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0(), 0.0]), g1: swizzle!(self.group4(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]), g2: Simd32x3::from(0.0), g3: self.group2() * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl GeometricAntiProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()[3]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: self.group2() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + self.group3() * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Origin {
    type Output = Origin;

    fn geometric_anti_product(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Flector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[3], -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Horizon> for Origin {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Line> for Origin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Origin {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_product(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Magnitude> for Origin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) } }
    }
}

impl GeometricAntiProduct<Motor> for Origin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Origin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], -other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Origin {
    type Output = AntiScalar;

    fn geometric_anti_product(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for Origin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Origin {
    type Output = LineAtOrigin;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for Origin {
    type Output = Translator;

    fn geometric_anti_product(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Origin {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for Origin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for Origin {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for Origin {
    type Output = Point;

    fn geometric_anti_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Plane {
    type Output = Plane;

    fn geometric_anti_product(self, other: AntiScalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Horizon> for Plane {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Line> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: self.group0() * Simd32x4::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group1()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Plane {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;

    fn geometric_anti_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Plane {
    type Output = Motor;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for Plane {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Plane {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Rotor> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for Plane {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Translator> for Plane {
    type Output = Flector;

    fn geometric_anti_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_anti_product(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], other.group1()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Horizon> for PlaneAtOrigin {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0()[0], -other.group0()[0], -other.group0()[0], 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl GeometricAntiProduct<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], other.group1()[2]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group1()[2], other.group4()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], -other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], -other.group0()[0], -other.group2()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_anti_product(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for PlaneAtOrigin {
    type Output = Motor;

    fn geometric_anti_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]), g1: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Rotor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for PlaneAtOrigin {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_anti_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Point {
    type Output = Point;

    fn geometric_anti_product(self, other: AntiScalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Point {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Horizon> for Point {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricAntiProduct<Line> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Point {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[1]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) } }
    }
}

impl GeometricAntiProduct<Motor> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group3()[0], -other.group3()[1], -other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], -other.group0()[0]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Point {
    type Output = Translator;

    fn geometric_anti_product(self, other: Origin) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) - self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for Point {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Point {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Point> for Point {
    type Output = Translator;

    fn geometric_anti_product(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Point {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for Point {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + swizzle!(self.group0(), 2, 2, 2, 3) * swizzle!(other.group0(), 1, 0, 3, 3) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]) } }
    }
}

impl GeometricAntiProduct<Scalar> for Point {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for Point {
    type Output = Point;

    fn geometric_anti_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: AntiScalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], -other.group0()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], -other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Line> for PointAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for PointAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Magnitude) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]) } }
    }
}

impl GeometricAntiProduct<Motor> for PointAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group4()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group4()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group4()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], -other.group2()[2], other.group2()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], -other.group2()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group2()[1], other.group2()[0], other.group0()[1], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], other.group4()[2], -other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group4()[2], -other.group1()[3], other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group4()[1], -other.group4()[0], -other.group1()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group2()[2]]) } }
    }
}

impl GeometricAntiProduct<Origin> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Origin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Point> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Point) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn geometric_anti_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group0()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Translator> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Translator) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Rotor {
    type Output = Rotor;

    fn geometric_anti_product(self, other: AntiScalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group0()[2], other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], other.group1()[3], -other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl GeometricAntiProduct<Horizon> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl GeometricAntiProduct<Line> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Rotor {
    type Output = Rotor;

    fn geometric_anti_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Motor> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], 0.0, -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], -other.group1()[2], other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], -other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], -other.group3()[2], other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], -other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group3()[1], other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() } }
    }
}

impl GeometricAntiProduct<Origin> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Origin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricAntiProduct<Plane> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricAntiProduct<Point> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn geometric_anti_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Rotor> for Rotor {
    type Output = Rotor;

    fn geometric_anti_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Scalar> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Translator> for Rotor {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: AntiScalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Flector> for Scalar {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]) } }
    }
}

impl GeometricAntiProduct<Line> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: Line) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: LineAtOrigin) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Magnitude> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Magnitude) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[1] } }
    }
}

impl GeometricAntiProduct<Motor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[1], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * other.group2(), g4: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) } }
    }
}

impl GeometricAntiProduct<Origin> for Scalar {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Origin) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricAntiProduct<Plane> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: Plane) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricAntiProduct<Point> for Scalar {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Point) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl GeometricAntiProduct<Rotor> for Scalar {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Translator> for Scalar {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Translator) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl GeometricAntiProduct<AntiScalar> for Translator {
    type Output = Translator;

    fn geometric_anti_product(self, other: AntiScalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Flector> for Translator {
    type Output = Flector;

    fn geometric_anti_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], other.group0()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * other.group1() } }
    }
}

impl GeometricAntiProduct<Horizon> for Translator {
    type Output = Horizon;

    fn geometric_anti_product(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricAntiProduct<Line> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<LineAtInfinity> for Translator {
    type Output = LineAtInfinity;

    fn geometric_anti_product(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<LineAtOrigin> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Magnitude> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[1]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * other.group1(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group2()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group2()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group2()[2], 0.0]) + Simd32x2::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], -other.group4()[2], other.group4()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], -other.group4()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group4()[1], other.group4()[0], other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group1(), g2: Simd32x3::from(self.group0()[3]) * other.group2(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], -other.group2()[2], other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], -other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group2()[1], other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * other.group4() } }
    }
}

impl GeometricAntiProduct<Origin> for Translator {
    type Output = Point;

    fn geometric_anti_product(self, other: Origin) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricAntiProduct<Plane> for Translator {
    type Output = Flector;

    fn geometric_anti_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn geometric_anti_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl GeometricAntiProduct<Point> for Translator {
    type Output = Point;

    fn geometric_anti_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<PointAtInfinity> for Translator {
    type Output = PointAtInfinity;

    fn geometric_anti_product(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricAntiProduct<Rotor> for Translator {
    type Output = MultiVector;

    fn geometric_anti_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from([self.group0()[2], self.group0()[3]]) * Simd32x2::from([-other.group0()[2], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], other.group0()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], other.group0()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricAntiProduct<Scalar> for Translator {
    type Output = Scalar;

    fn geometric_anti_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricAntiProduct<Translator> for Translator {
    type Output = Translator;

    fn geometric_anti_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricProduct<Flector> for AntiScalar {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Horizon> for AntiScalar {
    type Output = Origin;

    fn geometric_product(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Line> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Line) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<LineAtInfinity> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: LineAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: Magnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl GeometricProduct<Motor> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Motor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<Plane> for AntiScalar {
    type Output = Origin;

    fn geometric_product(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl GeometricProduct<Point> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Point) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: PointAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Translator> for AntiScalar {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<AntiScalar> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Flector> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Horizon> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Line> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group4()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl GeometricProduct<Origin> for Flector {
    type Output = Rotor;

    fn geometric_product(self, other: Origin) -> Rotor {
        Rotor { groups: RotorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl GeometricProduct<Plane> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Flector {
    type Output = Rotor;

    fn geometric_product(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Point> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn geometric_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Rotor> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Flector {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<AntiScalar> for Horizon {
    type Output = Origin;

    fn geometric_product(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Flector> for Horizon {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Horizon> for Horizon {
    type Output = Scalar;

    fn geometric_product(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Line> for Horizon {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn geometric_product(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<LineAtOrigin> for Horizon {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for Horizon {
    type Output = Flector;

    fn geometric_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) } }
    }
}

impl GeometricProduct<Motor> for Horizon {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for Horizon {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl GeometricProduct<Origin> for Horizon {
    type Output = AntiScalar;

    fn geometric_product(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Horizon {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Horizon {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Point> for Horizon {
    type Output = Translator;

    fn geometric_product(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn geometric_product(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Rotor> for Horizon {
    type Output = Flector;

    fn geometric_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Horizon {
    type Output = Horizon;

    fn geometric_product(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Translator> for Horizon {
    type Output = Point;

    fn geometric_product(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<AntiScalar> for Line {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Horizon> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Line> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Line {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Magnitude> for Line {
    type Output = Line;

    fn geometric_product(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<MultiVector> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl GeometricProduct<Origin> for Line {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn geometric_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Point> for Line {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Line {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for Line {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Line {
    type Output = Line;

    fn geometric_product(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Line {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<AntiScalar> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for LineAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Horizon> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_product(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtOrigin> for LineAtInfinity {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Magnitude> for LineAtInfinity {
    type Output = Line;

    fn geometric_product(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl GeometricProduct<Origin> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for LineAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for LineAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Point> for LineAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for LineAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for LineAtInfinity {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_product(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Flector> for LineAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) } }
    }
}

impl GeometricProduct<Horizon> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Horizon) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Line> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: Line) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricProduct<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) } }
    }
}

impl GeometricProduct<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Plane) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricProduct<Point> for LineAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Scalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for LineAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn geometric_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl GeometricProduct<Flector> for Magnitude {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Horizon> for Magnitude {
    type Output = Flector;

    fn geometric_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl GeometricProduct<Line> for Magnitude {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl GeometricProduct<LineAtInfinity> for Magnitude {
    type Output = Line;

    fn geometric_product(self, other: LineAtInfinity) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn geometric_product(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]) } }
    }
}

impl GeometricProduct<Motor> for Magnitude {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3(), g3: Simd32x3::from(self.group0()[0]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * other.group4() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<Origin> for Magnitude {
    type Output = Origin;

    fn geometric_product(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Magnitude {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Point> for Magnitude {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Magnitude {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Rotor> for Magnitude {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl GeometricProduct<Scalar> for Magnitude {
    type Output = Magnitude;

    fn geometric_product(self, other: Scalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Magnitude {
    type Output = Motor;

    fn geometric_product(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<AntiScalar> for Motor {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Horizon> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Line> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Motor {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Magnitude> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl GeometricProduct<Origin> for Motor {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Point> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Motor {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for Motor {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;

    fn geometric_product(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Motor {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]), g1: Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g2: self.group3() * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Flector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group1()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Horizon> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Line> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g2: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g2: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]), g1: self.group1() * Simd32x4::from(other.group0()[0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g2: self.group2() * Simd32x3::from(other.group0()[0]) + self.group3() * Simd32x3::from(other.group0()[1]), g3: self.group3() * Simd32x3::from(other.group0()[0]), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) + self.group4() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group4()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * other.group4() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl GeometricProduct<Origin> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[3]) * Simd32x2::from([0.0, -other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for MultiVector {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * other.group0(), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + self.group3() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Flector> for Origin {
    type Output = Rotor;

    fn geometric_product(self, other: Flector) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) } }
    }
}

impl GeometricProduct<Horizon> for Origin {
    type Output = AntiScalar;

    fn geometric_product(self, other: Horizon) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Line) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<LineAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: LineAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for Origin {
    type Output = Origin;

    fn geometric_product(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl GeometricProduct<Motor> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Motor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Origin {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group4()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]), g2: Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) } }
    }
}

impl GeometricProduct<Plane> for Origin {
    type Output = AntiScalar;

    fn geometric_product(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl GeometricProduct<Point> for Origin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Point) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: PointAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Scalar> for Origin {
    type Output = Origin;

    fn geometric_product(self, other: Scalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Translator> for Origin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Translator) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<AntiScalar> for Plane {
    type Output = Origin;

    fn geometric_product(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl GeometricProduct<Flector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Horizon> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Line> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl GeometricProduct<Origin> for Plane {
    type Output = AntiScalar;

    fn geometric_product(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Plane {
    type Output = MultiVector;

    fn geometric_product(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricProduct<Point> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Plane {
    type Output = Motor;

    fn geometric_product(self, other: PointAtInfinity) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl GeometricProduct<Rotor> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;

    fn geometric_product(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Plane {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Flector> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: Flector) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Horizon> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Horizon) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) } }
    }
}

impl GeometricProduct<Plane> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Plane) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricProduct<Point> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: Point) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for PlaneAtOrigin {
    type Output = Rotor;

    fn geometric_product(self, other: PointAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Scalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<AntiScalar> for Point {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Horizon> for Point {
    type Output = Translator;

    fn geometric_product(self, other: Horizon) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(-other.group0()) } }
    }
}

impl GeometricProduct<Line> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Point {
    type Output = Flector;

    fn geometric_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Point {
    type Output = Flector;

    fn geometric_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl GeometricProduct<Motor> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group4()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) } }
    }
}

impl GeometricProduct<Origin> for Point {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for Point {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Point {
    type Output = Rotor;

    fn geometric_product(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl GeometricProduct<Point> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Point {
    type Output = MultiVector;

    fn geometric_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Rotor> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Point {
    type Output = Point;

    fn geometric_product(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Point {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<AntiScalar> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Horizon> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn geometric_product(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Line> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<LineAtOrigin> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl GeometricProduct<Motor> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) } }
    }
}

impl GeometricProduct<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) } }
    }
}

impl GeometricProduct<Origin> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for PointAtInfinity {
    type Output = Motor;

    fn geometric_product(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for PointAtInfinity {
    type Output = Rotor;

    fn geometric_product(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl GeometricProduct<Point> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<PointAtInfinity> for PointAtInfinity {
    type Output = MultiVector;

    fn geometric_product(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn geometric_product(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for PointAtInfinity {
    type Output = Flector;

    fn geometric_product(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Flector> for Rotor {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Horizon> for Rotor {
    type Output = Flector;

    fn geometric_product(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl GeometricProduct<Line> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Line) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Magnitude> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Magnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<MultiVector> for Rotor {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<Plane> for Rotor {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) } }
    }
}

impl GeometricProduct<Point> for Rotor {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Scalar> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Rotor {
    type Output = Rotor;

    fn geometric_product(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn geometric_product(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Flector> for Scalar {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<Horizon> for Scalar {
    type Output = Horizon;

    fn geometric_product(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Line> for Scalar {
    type Output = Line;

    fn geometric_product(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn geometric_product(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Magnitude> for Scalar {
    type Output = Magnitude;

    fn geometric_product(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;

    fn geometric_product(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl GeometricProduct<Origin> for Scalar {
    type Output = Origin;

    fn geometric_product(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;

    fn geometric_product(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Point> for Scalar {
    type Output = Point;

    fn geometric_product(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn geometric_product(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Rotor> for Scalar {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;

    fn geometric_product(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl GeometricProduct<Translator> for Scalar {
    type Output = Translator;

    fn geometric_product(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl GeometricProduct<AntiScalar> for Translator {
    type Output = LineAtOrigin;

    fn geometric_product(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Flector> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<Horizon> for Translator {
    type Output = Point;

    fn geometric_product(self, other: Horizon) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Line> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<LineAtOrigin> for Translator {
    type Output = Rotor;

    fn geometric_product(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl GeometricProduct<Magnitude> for Translator {
    type Output = Motor;

    fn geometric_product(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) } }
    }
}

impl GeometricProduct<Motor> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl GeometricProduct<MultiVector> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl GeometricProduct<Origin> for Translator {
    type Output = PlaneAtOrigin;

    fn geometric_product(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl GeometricProduct<Plane> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl GeometricProduct<Point> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl GeometricProduct<PointAtInfinity> for Translator {
    type Output = Flector;

    fn geometric_product(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) - swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[2]]) } }
    }
}

impl GeometricProduct<Rotor> for Translator {
    type Output = Rotor;

    fn geometric_product(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl GeometricProduct<Scalar> for Translator {
    type Output = Translator;

    fn geometric_product(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl GeometricProduct<Translator> for Translator {
    type Output = MultiVector;

    fn geometric_product(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Flector> for AntiScalar {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Horizon> for AntiScalar {
    type Output = Origin;

    fn wedge_dot(self, other: Horizon) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl WedgeDot<Line> for AntiScalar {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Line) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<LineAtInfinity> for AntiScalar {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: LineAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for AntiScalar {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Magnitude) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl WedgeDot<Motor> for AntiScalar {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Motor) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<MultiVector> for AntiScalar {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<Plane> for AntiScalar {
    type Output = Origin;

    fn wedge_dot(self, other: Plane) -> Origin {
        Origin { groups: OriginGroups { g0: 0.0 - self.group0() * other.group0()[3] } }
    }
}

impl WedgeDot<Point> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Point) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for AntiScalar {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: PointAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Scalar> for AntiScalar {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Scalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Translator> for AntiScalar {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Translator) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<AntiScalar> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: AntiScalar) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Flector> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Horizon> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Line> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<LineAtOrigin> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) + self.group1() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group4()[3]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl WedgeDot<Origin> for Flector {
    type Output = Rotor;

    fn wedge_dot(self, other: Origin) -> Rotor {
        Rotor { groups: RotorGroups { g0: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) } }
    }
}

impl WedgeDot<Plane> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, other.group0()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Flector {
    type Output = Rotor;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Point> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, -other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PointAtInfinity> for Flector {
    type Output = MultiVector;

    fn wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group1()[3]) * other.group0(), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Rotor> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Scalar) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Flector {
    type Output = Flector;

    fn wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<AntiScalar> for Horizon {
    type Output = Origin;

    fn wedge_dot(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Flector> for Horizon {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Horizon> for Horizon {
    type Output = Scalar;

    fn wedge_dot(self, other: Horizon) -> Scalar {
        Scalar { groups: ScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl WedgeDot<Line> for Horizon {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for Horizon {
    type Output = PointAtInfinity;

    fn wedge_dot(self, other: LineAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<LineAtOrigin> for Horizon {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for Horizon {
    type Output = Flector;

    fn wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) } }
    }
}

impl WedgeDot<Motor> for Horizon {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for Horizon {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl WedgeDot<Origin> for Horizon {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0() * other.group0() } }
    }
}

impl WedgeDot<Plane> for Horizon {
    type Output = MultiVector;

    fn wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Horizon {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Point> for Horizon {
    type Output = Translator;

    fn wedge_dot(self, other: Point) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(0.0) - Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<PointAtInfinity> for Horizon {
    type Output = LineAtInfinity;

    fn wedge_dot(self, other: PointAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Rotor> for Horizon {
    type Output = Flector;

    fn wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for Horizon {
    type Output = Horizon;

    fn wedge_dot(self, other: Scalar) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Translator> for Horizon {
    type Output = Point;

    fn wedge_dot(self, other: Translator) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<AntiScalar> for Line {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Horizon> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Line> for Line {
    type Output = MultiVector;

    fn wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtInfinity> for Line {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtOrigin> for Line {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Magnitude> for Line {
    type Output = Line;

    fn wedge_dot(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) + self.group1() * Simd32x3::from(other.group0()[1]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Line {
    type Output = MultiVector;

    fn wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<MultiVector> for Line {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl WedgeDot<Origin> for Line {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Point> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Line {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Rotor> for Line {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl WedgeDot<Scalar> for Line {
    type Output = Line;

    fn wedge_dot(self, other: Scalar) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Line {
    type Output = MultiVector;

    fn wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<AntiScalar> for LineAtInfinity {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for LineAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Horizon> for LineAtInfinity {
    type Output = PointAtInfinity;

    fn wedge_dot(self, other: Horizon) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Line> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtInfinity> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtOrigin> for LineAtInfinity {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Magnitude> for LineAtInfinity {
    type Output = Line;

    fn wedge_dot(self, other: Magnitude) -> Line {
        Line { groups: LineGroups { g0: self.group0() * Simd32x3::from(other.group0()[1]), g1: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<MultiVector> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl WedgeDot<Origin> for LineAtInfinity {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for LineAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for LineAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Point> for LineAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for LineAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Rotor> for LineAtInfinity {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl WedgeDot<Scalar> for LineAtInfinity {
    type Output = LineAtInfinity;

    fn wedge_dot(self, other: Scalar) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for LineAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Flector> for LineAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) } }
    }
}

impl WedgeDot<Horizon> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Horizon) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Line> for LineAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: Line) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl WedgeDot<LineAtInfinity> for LineAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Magnitude> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Magnitude) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for LineAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl WedgeDot<MultiVector> for LineAtOrigin {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) } }
    }
}

impl WedgeDot<Plane> for LineAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Plane) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl WedgeDot<Point> for LineAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PointAtInfinity> for LineAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Scalar> for LineAtOrigin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Scalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for LineAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<AntiScalar> for Magnitude {
    type Output = AntiScalar;

    fn wedge_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl WedgeDot<Flector> for Magnitude {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Horizon> for Magnitude {
    type Output = Flector;

    fn wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) } }
    }
}

impl WedgeDot<Line> for Magnitude {
    type Output = Line;

    fn wedge_dot(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1(), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl WedgeDot<LineAtInfinity> for Magnitude {
    type Output = Line;

    fn wedge_dot(self, other: LineAtInfinity) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()[1]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl WedgeDot<LineAtOrigin> for Magnitude {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for Magnitude {
    type Output = Magnitude;

    fn wedge_dot(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]) } }
    }
}

impl WedgeDot<Motor> for Magnitude {
    type Output = Motor;

    fn wedge_dot(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]), g1: Simd32x3::from(self.group0()[0]) * other.group1() } }
    }
}

impl WedgeDot<MultiVector> for Magnitude {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3(), g3: Simd32x3::from(self.group0()[0]) * other.group3(), g4: Simd32x4::from(self.group0()[0]) * other.group4() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<Origin> for Magnitude {
    type Output = Origin;

    fn wedge_dot(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[0] * other.group0() } }
    }
}

impl WedgeDot<Plane> for Magnitude {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Magnitude {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[0]) * other.group0() } }
    }
}

impl WedgeDot<Point> for Magnitude {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0(), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Magnitude {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Rotor> for Magnitude {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * other.group0() } }
    }
}

impl WedgeDot<Scalar> for Magnitude {
    type Output = Magnitude;

    fn wedge_dot(self, other: Scalar) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: self.group0() * Simd32x2::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Magnitude {
    type Output = Motor;

    fn wedge_dot(self, other: Translator) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group0(), g1: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<AntiScalar> for Motor {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Horizon> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Line> for Motor {
    type Output = MultiVector;

    fn wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtInfinity> for Motor {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtOrigin> for Motor {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Magnitude> for Motor {
    type Output = Motor;

    fn wedge_dot(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]), g1: self.group1() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Motor {
    type Output = MultiVector;

    fn wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<MultiVector> for Motor {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]) - Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) } }
    }
}

impl WedgeDot<Origin> for Motor {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Point> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Motor {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Rotor> for Motor {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group1()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group1()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group1()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl WedgeDot<Scalar> for Motor {
    type Output = Motor;

    fn wedge_dot(self, other: Scalar) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from(other.group0()), g1: self.group1() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Motor {
    type Output = MultiVector;

    fn wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group1() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<AntiScalar> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: AntiScalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()]), g1: Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g2: self.group3() * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Flector> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group1()[3]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Horizon> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]) + Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]), g2: Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]) + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Line> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g2: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group4(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g2: Simd32x3::from(self.group0()[1]) * other.group0() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group1(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<LineAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g2: Simd32x3::from(self.group0()[0]) * other.group0() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Magnitude) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]), g1: self.group1() * Simd32x4::from(other.group0()[0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g2: self.group2() * Simd32x3::from(other.group0()[0]) + self.group3() * Simd32x3::from(other.group0()[1]), g3: self.group3() * Simd32x3::from(other.group0()[0]), g4: swizzle!(self.group1(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) + self.group4() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group0()[1]) * other.group1() + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * other.group1() + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * other.group0() + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group4()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) - Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group1() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * other.group2() + Simd32x3::from(self.group0()[1]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(self.group0()[0]) * other.group3() + Simd32x3::from(self.group1()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * other.group4() + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl WedgeDot<Origin> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Origin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group4()[3]) * Simd32x2::from([0.0, -other.group0()]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group3()[0]]) * Simd32x4::from([other.group0(), other.group0(), other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Plane> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]) + Simd32x2::from(self.group1()[3]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) + Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]), g4: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[0]]) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: PlaneAtOrigin) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([0.0, other.group0()[0]]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([0.0, other.group0()[1]]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([0.0, other.group0()[2]]), g1: Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g2: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Point> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group4()[3]) * Simd32x2::from([0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * other.group0() + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], 0.0]) + Simd32x2::from(self.group4()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group4()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group4()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g2: Simd32x3::from(self.group1()[3]) * other.group0() + Simd32x3::from(self.group4()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group4()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g3: Simd32x3::from(self.group1()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group1()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]) - Simd32x3::from(self.group4()[3]) * other.group0(), g4: Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) + Simd32x4::from(self.group2()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group2()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Rotor> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Rotor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Scalar) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: self.group0() * Simd32x2::from(other.group0()), g1: self.group1() * Simd32x4::from(other.group0()), g2: self.group2() * Simd32x3::from(other.group0()), g3: self.group3() * Simd32x3::from(other.group0()), g4: self.group4() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for MultiVector {
    type Output = MultiVector;

    fn wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, other.group0()[3]]) + Simd32x2::from(self.group2()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group2()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group2()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) + Simd32x2::from(self.group3()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group3()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group3()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group4()[3]) * other.group0(), g2: Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group2()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group2()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + self.group3() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + Simd32x3::from(self.group3()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group3()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) + Simd32x4::from(self.group4()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group4()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Flector> for Origin {
    type Output = Rotor;

    fn wedge_dot(self, other: Flector) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) } }
    }
}

impl WedgeDot<Horizon> for Origin {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Horizon) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Line> for Origin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Line) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<LineAtInfinity> for Origin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: LineAtInfinity) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for Origin {
    type Output = Origin;

    fn wedge_dot(self, other: Magnitude) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0()[0] } }
    }
}

impl WedgeDot<Motor> for Origin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Motor) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<MultiVector> for Origin {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * Simd32x2::from([0.0, other.group4()[3]]), g1: Simd32x4::from(self.group0()) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]), g2: Simd32x3::from(self.group0()) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) } }
    }
}

impl WedgeDot<Plane> for Origin {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Plane) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0()[3] } }
    }
}

impl WedgeDot<Point> for Origin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Point) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Origin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: PointAtInfinity) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Scalar> for Origin {
    type Output = Origin;

    fn wedge_dot(self, other: Scalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Translator> for Origin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Translator) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<AntiScalar> for Plane {
    type Output = Origin;

    fn wedge_dot(self, other: AntiScalar) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0()[3] * other.group0() } }
    }
}

impl WedgeDot<Flector> for Plane {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group0()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group0()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group0()[2]]) - Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group1()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group1()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Horizon> for Plane {
    type Output = MultiVector;

    fn wedge_dot(self, other: Horizon) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0(), 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Line> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<LineAtOrigin> for Plane {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: LineAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]), g1: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for Plane {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]) - Simd32x2::from(self.group0()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]), g3: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) } }
    }
}

impl WedgeDot<Origin> for Plane {
    type Output = AntiScalar;

    fn wedge_dot(self, other: Origin) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: 0.0 - self.group0()[3] * other.group0() } }
    }
}

impl WedgeDot<Plane> for Plane {
    type Output = MultiVector;

    fn wedge_dot(self, other: Plane) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[3]) * Simd32x2::from([-other.group0()[3], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Plane {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: PlaneAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl WedgeDot<Point> for Plane {
    type Output = Motor;

    fn wedge_dot(self, other: Point) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Plane {
    type Output = Motor;

    fn wedge_dot(self, other: PointAtInfinity) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]), g1: Simd32x3::from(0.0) - Simd32x3::from(self.group0()[3]) * other.group0() } }
    }
}

impl WedgeDot<Rotor> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for Plane {
    type Output = Plane;

    fn wedge_dot(self, other: Scalar) -> Plane {
        Plane { groups: PlaneGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Plane {
    type Output = Flector;

    fn wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * other.group0(), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Flector> for PlaneAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: Flector) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Horizon> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Horizon) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Line> for PlaneAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for PlaneAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Magnitude) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for PlaneAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for PlaneAtOrigin {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group1()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group3()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], other.group4()[3], other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], other.group4()[3]]), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], 0.0]) } }
    }
}

impl WedgeDot<Plane> for PlaneAtOrigin {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Plane) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl WedgeDot<Point> for PlaneAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: Point) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<PointAtInfinity> for PlaneAtOrigin {
    type Output = Rotor;

    fn wedge_dot(self, other: PointAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Scalar> for PlaneAtOrigin {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Scalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for PlaneAtOrigin {
    type Output = Flector;

    fn wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<AntiScalar> for Point {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for Point {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group1()[3]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Horizon> for Point {
    type Output = Translator;

    fn wedge_dot(self, other: Horizon) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(-other.group0()) } }
    }
}

impl WedgeDot<Line> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], -other.group0()[2]]) } }
    }
}

impl WedgeDot<LineAtOrigin> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl WedgeDot<Motor> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for Point {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group4()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], 0.0]) } }
    }
}

impl WedgeDot<Origin> for Point {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for Point {
    type Output = Motor;

    fn wedge_dot(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]), g1: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Point {
    type Output = Rotor;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl WedgeDot<Point> for Point {
    type Output = MultiVector;

    fn wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PointAtInfinity> for Point {
    type Output = MultiVector;

    fn wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Rotor> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for Point {
    type Output = Point;

    fn wedge_dot(self, other: Scalar) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Point {
    type Output = Flector;

    fn wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<AntiScalar> for PointAtInfinity {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for PointAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: Flector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group0()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group0()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group0()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], -other.group1()[3], -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], -other.group1()[3]]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Horizon> for PointAtInfinity {
    type Output = LineAtInfinity;

    fn wedge_dot(self, other: Horizon) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Line> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Line) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group1()[2]]) } }
    }
}

impl WedgeDot<LineAtInfinity> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<LineAtOrigin> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: LineAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Magnitude) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], 0.0]), g1: Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group0()[0]]) * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], 0.0]) } }
    }
}

impl WedgeDot<Motor> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Motor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], -other.group1()[2]]) } }
    }
}

impl WedgeDot<MultiVector> for PointAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], -other.group3()[1], -other.group2()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group3()[2], other.group0()[0], other.group3()[0], -other.group2()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group3()[1], -other.group3()[0], other.group0()[0], -other.group2()[2]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group1()[3], -other.group4()[2], other.group4()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group4()[2], -other.group1()[3], -other.group4()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group4()[1], other.group4()[0], -other.group1()[3]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([-other.group4()[3], -other.group1()[2], other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], -other.group4()[3], -other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group1()[1], other.group1()[0], -other.group4()[3]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], -other.group2()[1], -other.group3()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group2()[2], other.group0()[1], other.group2()[0], -other.group3()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group2()[1], -other.group2()[0], other.group0()[1], -other.group3()[2]]) } }
    }
}

impl WedgeDot<Origin> for PointAtInfinity {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: Origin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for PointAtInfinity {
    type Output = Motor;

    fn wedge_dot(self, other: Plane) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]), g1: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for PointAtInfinity {
    type Output = Rotor;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, other.group0()[2]]) } }
    }
}

impl WedgeDot<Point> for PointAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: Point) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0) - self.group0() * Simd32x3::from(other.group0()[3]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<PointAtInfinity> for PointAtInfinity {
    type Output = MultiVector;

    fn wedge_dot(self, other: PointAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(0.0), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, -other.group0()[2], other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[2], 0.0, -other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([-other.group0()[1], other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<Rotor> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Rotor) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group0()[3], other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group0()[3], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for PointAtInfinity {
    type Output = PointAtInfinity;

    fn wedge_dot(self, other: Scalar) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: self.group0() * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for PointAtInfinity {
    type Output = Flector;

    fn wedge_dot(self, other: Translator) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) } }
    }
}

impl WedgeDot<Flector> for Rotor {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group1()[3], -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], -other.group1()[3], -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], -other.group1()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Horizon> for Rotor {
    type Output = Flector;

    fn wedge_dot(self, other: Horizon) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0(), -other.group0(), -other.group0(), 0.0]) } }
    }
}

impl WedgeDot<Line> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: Line) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<LineAtInfinity> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtInfinity) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Magnitude> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: Magnitude) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: Motor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group1()[2], -other.group1()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], 0.0, other.group1()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], 0.0, -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<MultiVector> for Rotor {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([0.0, -other.group3()[0]]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([0.0, -other.group3()[1]]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([0.0, -other.group3()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g3: Simd32x3::from(0.0), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([-other.group4()[3], -other.group1()[2], other.group1()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group1()[2], -other.group4()[3], -other.group1()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group1()[1], other.group1()[0], -other.group4()[3], 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<Plane> for Rotor {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: swizzle!(self.group0(), 0, 1, 2, 0) * Simd32x4::from([-other.group0()[3], -other.group0()[3], -other.group0()[3], 0.0]) } }
    }
}

impl WedgeDot<Point> for Rotor {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Rotor {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, -other.group0()[2], other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[2], 0.0, -other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([-other.group0()[1], other.group0()[0], 0.0, 0.0]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Scalar> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: Scalar) -> Rotor {
        Rotor { groups: RotorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Rotor {
    type Output = Rotor;

    fn wedge_dot(self, other: Translator) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<AntiScalar> for Scalar {
    type Output = AntiScalar;

    fn wedge_dot(self, other: AntiScalar) -> AntiScalar {
        AntiScalar { groups: AntiScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Flector> for Scalar {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<Horizon> for Scalar {
    type Output = Horizon;

    fn wedge_dot(self, other: Horizon) -> Horizon {
        Horizon { groups: HorizonGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Line> for Scalar {
    type Output = Line;

    fn wedge_dot(self, other: Line) -> Line {
        Line { groups: LineGroups { g0: Simd32x3::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<LineAtInfinity> for Scalar {
    type Output = LineAtInfinity;

    fn wedge_dot(self, other: LineAtInfinity) -> LineAtInfinity {
        LineAtInfinity { groups: LineAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<LineAtOrigin> for Scalar {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: LineAtOrigin) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Magnitude> for Scalar {
    type Output = Magnitude;

    fn wedge_dot(self, other: Magnitude) -> Magnitude {
        Magnitude { groups: MagnitudeGroups { g0: Simd32x2::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Motor> for Scalar {
    type Output = Motor;

    fn wedge_dot(self, other: Motor) -> Motor {
        Motor { groups: MotorGroups { g0: Simd32x4::from(self.group0()) * other.group0(), g1: Simd32x3::from(self.group0()) * other.group1() } }
    }
}

impl WedgeDot<MultiVector> for Scalar {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()) * other.group0(), g1: Simd32x4::from(self.group0()) * other.group1(), g2: Simd32x3::from(self.group0()) * other.group2(), g3: Simd32x3::from(self.group0()) * other.group3(), g4: Simd32x4::from(self.group0()) * other.group4() } }
    }
}

impl WedgeDot<Origin> for Scalar {
    type Output = Origin;

    fn wedge_dot(self, other: Origin) -> Origin {
        Origin { groups: OriginGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Plane> for Scalar {
    type Output = Plane;

    fn wedge_dot(self, other: Plane) -> Plane {
        Plane { groups: PlaneGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Scalar {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: PlaneAtOrigin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Point> for Scalar {
    type Output = Point;

    fn wedge_dot(self, other: Point) -> Point {
        Point { groups: PointGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<PointAtInfinity> for Scalar {
    type Output = PointAtInfinity;

    fn wedge_dot(self, other: PointAtInfinity) -> PointAtInfinity {
        PointAtInfinity { groups: PointAtInfinityGroups { g0: Simd32x3::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Rotor> for Scalar {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<Scalar> for Scalar {
    type Output = Scalar;

    fn wedge_dot(self, other: Scalar) -> Scalar {
        Scalar { groups: ScalarGroups { g0: self.group0() * other.group0() } }
    }
}

impl WedgeDot<Translator> for Scalar {
    type Output = Translator;

    fn wedge_dot(self, other: Translator) -> Translator {
        Translator { groups: TranslatorGroups { g0: Simd32x4::from(self.group0()) * other.group0() } }
    }
}

impl WedgeDot<AntiScalar> for Translator {
    type Output = LineAtOrigin;

    fn wedge_dot(self, other: AntiScalar) -> LineAtOrigin {
        LineAtOrigin { groups: LineAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Flector> for Translator {
    type Output = Flector;

    fn wedge_dot(self, other: Flector) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group0()[2], -other.group0()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], other.group1()[3], other.group0()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group1()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], other.group1()[2], -other.group1()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group0()[3], other.group1()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<Horizon> for Translator {
    type Output = Point;

    fn wedge_dot(self, other: Horizon) -> Point {
        Point { groups: PointGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Line> for Translator {
    type Output = MultiVector;

    fn wedge_dot(self, other: Line) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]) + Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtInfinity> for Translator {
    type Output = MultiVector;

    fn wedge_dot(self, other: LineAtInfinity) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[3]) * other.group0(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<LineAtOrigin> for Translator {
    type Output = Rotor;

    fn wedge_dot(self, other: LineAtOrigin) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, -other.group0()[2]]) } }
    }
}

impl WedgeDot<Magnitude> for Translator {
    type Output = Motor;

    fn wedge_dot(self, other: Magnitude) -> Motor {
        Motor { groups: MotorGroups { g0: self.group0() * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]), g1: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[0]) } }
    }
}

impl WedgeDot<Motor> for Translator {
    type Output = MultiVector;

    fn wedge_dot(self, other: Motor) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]), g1: Simd32x4::from(0.0), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[3], other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], other.group0()[3], other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], other.group0()[3]]) + Simd32x3::from(self.group0()[3]) * other.group1(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group1()[2], -other.group1()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group1()[2], 0.0, other.group1()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group1()[1], -other.group1()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

impl WedgeDot<MultiVector> for Translator {
    type Output = MultiVector;

    fn wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(0.0) - Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) - Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) - Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) + Simd32x2::from(self.group0()[3]) * Simd32x2::from([0.0, other.group0()[0]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], -other.group1()[1], -other.group4()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group1()[2], other.group4()[3], other.group1()[0], -other.group4()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group1()[1], -other.group1()[0], other.group4()[3], -other.group4()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group4()[3]]), g2: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], -other.group2()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group2()[2], other.group0()[1], other.group2()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group2()[1], -other.group2()[0], other.group0()[1]]) + Simd32x3::from(self.group0()[3]) * other.group3(), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], -other.group3()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group3()[2], other.group0()[0], other.group3()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group3()[1], -other.group3()[0], other.group0()[0]]), g4: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], -other.group4()[1], -other.group1()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group4()[2], other.group1()[3], other.group4()[0], -other.group1()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group4()[1], -other.group4()[0], other.group1()[3], -other.group1()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group1()[0], -other.group1()[1], -other.group1()[2], 0.0]) } }
    }
}

impl WedgeDot<Origin> for Translator {
    type Output = PlaneAtOrigin;

    fn wedge_dot(self, other: Origin) -> PlaneAtOrigin {
        PlaneAtOrigin { groups: PlaneAtOriginGroups { g0: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()) } }
    }
}

impl WedgeDot<Plane> for Translator {
    type Output = Flector;

    fn wedge_dot(self, other: Plane) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[3]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<PlaneAtOrigin> for Translator {
    type Output = Flector;

    fn wedge_dot(self, other: PlaneAtOrigin) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[2]]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]) } }
    }
}

impl WedgeDot<Point> for Translator {
    type Output = Flector;

    fn wedge_dot(self, other: Point) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[3], 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, other.group0()[3], 0.0, -other.group0()[1]]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([0.0, 0.0, other.group0()[3], -other.group0()[2]]) + Simd32x4::from(self.group0()[3]) * Simd32x4::from([-other.group0()[0], -other.group0()[1], -other.group0()[2], 0.0]) } }
    }
}

impl WedgeDot<PointAtInfinity> for Translator {
    type Output = Flector;

    fn wedge_dot(self, other: PointAtInfinity) -> Flector {
        Flector { groups: FlectorGroups { g0: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, other.group0()[2], -other.group0()[1], 0.0]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([-other.group0()[2], 0.0, other.group0()[0], 0.0]) + Simd32x4::from(self.group0()[2]) * Simd32x4::from([other.group0()[1], -other.group0()[0], 0.0, 0.0]), g1: Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[0]]) + Simd32x4::from(self.group0()[1]) * Simd32x4::from([0.0, 0.0, 0.0, -other.group0()[1]]) - swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group0()[2]]) } }
    }
}

impl WedgeDot<Rotor> for Translator {
    type Output = Rotor;

    fn wedge_dot(self, other: Rotor) -> Rotor {
        Rotor { groups: RotorGroups { g0: Simd32x4::from(self.group0()[0]) * swizzle!(other.group0(), 3, 2, 1, 0) * Simd32x4::from([1.0, 1.0, -1.0, -1.0]) + Simd32x4::from(self.group0()[1]) * swizzle!(other.group0(), 2, 3, 0, 1) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0]) + Simd32x4::from(self.group0()[2]) * swizzle!(other.group0(), 1, 0, 3, 2) * Simd32x4::from([1.0, -1.0, 1.0, -1.0]) } }
    }
}

impl WedgeDot<Scalar> for Translator {
    type Output = Translator;

    fn wedge_dot(self, other: Scalar) -> Translator {
        Translator { groups: TranslatorGroups { g0: self.group0() * Simd32x4::from(other.group0()) } }
    }
}

impl WedgeDot<Translator> for Translator {
    type Output = MultiVector;

    fn wedge_dot(self, other: Translator) -> MultiVector {
        MultiVector { groups: MultiVectorGroups { g0: Simd32x2::from(self.group0()[0]) * Simd32x2::from([-other.group0()[0], 0.0]) + Simd32x2::from(self.group0()[1]) * Simd32x2::from([-other.group0()[1], 0.0]) + Simd32x2::from(self.group0()[2]) * Simd32x2::from([-other.group0()[2], 0.0]), g1: Simd32x4::from(0.0), g2: Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(other.group0()[3]) + Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]), g3: Simd32x3::from(self.group0()[0]) * Simd32x3::from([0.0, other.group0()[2], -other.group0()[1]]) + Simd32x3::from(self.group0()[1]) * Simd32x3::from([-other.group0()[2], 0.0, other.group0()[0]]) + Simd32x3::from(self.group0()[2]) * Simd32x3::from([other.group0()[1], -other.group0()[0], 0.0]), g4: Simd32x4::from(0.0) } }
    }
}

