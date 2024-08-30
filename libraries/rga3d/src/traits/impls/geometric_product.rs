// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 117
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       4       0
//  Average:         6      11       0
//  Maximum:       103     118       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       8       0
//  Average:        10      18       0
//  Maximum:       177     192       0
impl InfixGeometricProduct for AntiScalar {}
impl GeometricProduct<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other.group0()[0] * self[e1234]));
    }
}
impl GeometricProduct<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * self[e1234] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self[e1234] * -1.0),
                (other.group0()[1] * self[e1234] * -1.0),
                (other.group0()[2] * self[e1234] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Horizon> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e1234] * other[e321] * -1.0));
    }
}
impl GeometricProduct<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ (Simd32x3::from(self[e1234]) * other.group1()), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl GeometricProduct<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e1234]) * other.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other.group0()[0] * self[e1234])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group4()[3] * self[e1234] * -1.0)]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group1()[0] * self[e1234] * -1.0),
                (other.group1()[1] * self[e1234] * -1.0),
                (other.group1()[2] * self[e1234] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Plane> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (other.group0()[3] * self[e1234] * -1.0));
    }
}
impl GeometricProduct<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([
            (other.group0()[0] * self[e1234] * -1.0),
            (other.group0()[1] * self[e1234] * -1.0),
            (other.group0()[2] * self[e1234] * -1.0),
            0.0,
        ]));
    }
}
impl GeometricProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e1234] * other[scalar]));
    }
}
impl InfixGeometricProduct for DualNum {}
impl GeometricProduct<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[0] * other[e1234]));
    }
}
impl GeometricProduct<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (other.group0()[0] * self.group0()[0]),
            ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
        ]));
    }
}
impl GeometricProduct<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                ((self.group0()[0] * other.group0()[3]) - (self.group0()[1] * other.group1()[3])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group0()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group0()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[1] * other.group0()[2])),
                (self.group0()[0] * other.group1()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other[e321] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e321])]),
        );
    }
}
impl GeometricProduct<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(self.group0()[1]) * other.group1())),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group1()),
        );
    }
}
impl GeometricProduct<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group0()[0]) * other.group0()) + (Simd32x4::from(self.group0()[1]) * other.group1())),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[0]) * other.group1()),
        );
    }
}
impl GeometricProduct<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0]),
                (self.group0()[0] * other.group1()[1]),
                (self.group0()[0] * other.group1()[2]),
                ((self.group0()[0] * other.group1()[3]) - (self.group0()[1] * other.group4()[3])),
            ]),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group2()) + (Simd32x3::from(self.group0()[1]) * other.group3())),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group4()[0]) - (self.group0()[1] * other.group1()[0])),
                ((self.group0()[0] * other.group4()[1]) - (self.group0()[1] * other.group1()[1])),
                ((self.group0()[0] * other.group4()[2]) - (self.group0()[1] * other.group1()[2])),
                (self.group0()[0] * other.group4()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self.group0()[0] * other[e4]));
    }
}
impl GeometricProduct<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other.group0()[3] * -1.0)]),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[0]) * other.group0()),
        );
    }
}
impl GeometricProduct<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0] * -1.0),
                (self.group0()[1] * other.group0()[1] * -1.0),
                (self.group0()[1] * other.group0()[2] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(other[scalar]) * self.group0()));
    }
}
impl InfixGeometricProduct for Flector {}
impl GeometricProduct<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[e1234])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * other[e1234]), (self.group0()[1] * other[e1234]), (self.group0()[2] * other[e1234]), 0.0]),
        );
    }
}
impl GeometricProduct<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group0()[0]),
                (other.group0()[0] * self.group0()[1]),
                (other.group0()[0] * self.group0()[2]),
                ((other.group0()[0] * self.group0()[3]) + (other.group0()[1] * self.group1()[3])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group0()[1])),
                ((other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group0()[2])),
                (other.group0()[0] * self.group1()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       16       24        0
    //  no simd       40       48        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) - (other.group1()[1] * self.group0()[2])),
                ((other.group0()[2] * self.group1()[0]) - (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) - (other.group1()[0] * self.group0()[1])),
                (-(other.group0()[3] * self.group1()[3]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[2]]) * swizzle!(self.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * swizzle!(other.group1(), 3, 3, 3, 2))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 2, 0, 1, 1) * swizzle!(self.group0(), 1, 2, 0, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) - (other.group1()[3] * self.group0()[0])),
                (-(other.group0()[2] * self.group0()[0]) - (other.group1()[3] * self.group0()[1])),
                (-(other.group0()[0] * self.group0()[1]) - (other.group1()[3] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl GeometricProduct<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group1()[0] * self.group0()[1]) + (other.group1()[2] * self.group1()[3])),
                (-(other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[1] * self.group1()[2])
                    - (other.group1()[2] * self.group1()[1])),
                ((other.group0()[1] * self.group1()[3]) + (other.group0()[2] * self.group0()[0]) - (other.group1()[0] * self.group1()[2])
                    + (other.group1()[1] * self.group0()[3])
                    + (other.group1()[2] * self.group1()[0])),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[0] * self.group1()[1]) - (other.group1()[1] * self.group1()[0])
                    + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       40       49        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (self.group0()[2] * other.group1()[1]),
                (self.group0()[1] * other.group1()[3]),
                (self.group0()[2] * other.group1()[3]),
                (-(self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (swizzle!(self.group0(), 0, 0, 1, 3) * swizzle!(other.group1(), 3, 2, 0, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3])
                    + (self.group0()[2] * other.group0()[1])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[1])
                    + (self.group1()[3] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[2])
                    + (self.group0()[1] * other.group0()[3])
                    + (self.group1()[0] * other.group1()[2])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[3] * other.group0()[1])),
                ((self.group0()[1] * other.group0()[0])
                    + (self.group0()[2] * other.group0()[3])
                    + (self.group1()[1] * other.group1()[0])
                    + (self.group1()[2] * other.group1()[3])
                    + (self.group1()[3] * other.group0()[2])),
                ((self.group0()[2] * other.group1()[2]) * -1.0),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]) * other.group1())
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[1]]) * swizzle!(other.group1(), 2, 0, 1, 1))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       50       65        0
    //  no simd       81       96        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((self.group0()[3] * other.group4()[3]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                - (Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group3()[1] * self.group0()[2]),
                (other.group3()[2] * self.group0()[0]),
                (other.group3()[0] * self.group0()[1]),
                (-(other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    - (other.group3()[0] * self.group1()[0])
                    - (other.group3()[1] * self.group1()[1])
                    - (other.group3()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group0())
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((self.group0()[1] * other.group4()[2]) - (self.group0()[2] * other.group4()[1]) - (self.group1()[1] * other.group1()[2]) + (self.group1()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group4()[2]) + (self.group0()[2] * other.group4()[0]) + (self.group1()[0] * other.group1()[2])
                    - (self.group1()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group4()[1]) - (self.group0()[1] * other.group4()[0]) - (self.group1()[0] * other.group1()[1]) + (self.group1()[1] * other.group1()[0])),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
            ]) - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[0])
                    + (other.group2()[0] * self.group1()[3])
                    + (other.group2()[1] * self.group0()[2])
                    + (other.group3()[0] * self.group0()[3])
                    + (other.group3()[1] * self.group1()[2])
                    - (other.group3()[2] * self.group1()[1])),
                ((other.group0()[1] * self.group0()[1]) + (other.group2()[1] * self.group1()[3]) + (other.group2()[2] * self.group0()[0]) - (other.group3()[0] * self.group1()[2])
                    + (other.group3()[1] * self.group0()[3])
                    + (other.group3()[2] * self.group1()[0])),
                ((other.group0()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[1]) + (other.group2()[2] * self.group1()[3]) + (other.group3()[0] * self.group1()[1])
                    - (other.group3()[1] * self.group1()[0])
                    + (other.group3()[2] * self.group0()[3])),
                (-(other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(self.group0()[2] * other.group0()[1]) - (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[2]) - (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[0]) - (self.group1()[3] * other.group0()[2])),
                ((self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3])),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl GeometricProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[2] * other.group0()[1])),
                ((self.group0()[3] * other.group0()[1]) + (self.group1()[0] * other.group0()[2])),
                ((self.group0()[3] * other.group0()[2]) + (self.group1()[1] * other.group0()[0])),
                (-(self.group1()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3])),
            ]) - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * swizzle!(other.group0(), 2, 0, 1, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(self.group0()[2] * other.group0()[1]) - (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[2]) - (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[0]) - (self.group1()[3] * other.group0()[2])),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])),
            ]) + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
        );
    }
}
impl GeometricProduct<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group1()),
        );
    }
}
impl InfixGeometricProduct for Horizon {}
impl GeometricProduct<AntiScalar> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (other[e1234] * self[e321]));
    }
}
impl GeometricProduct<DualNum> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self[e321])]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self[e321])]),
        );
    }
}
impl GeometricProduct<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl GeometricProduct<Horizon> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e321] * self[e321] * -1.0));
    }
}
impl GeometricProduct<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * self[e321]), (other.group1()[1] * self[e321]), (other.group1()[2] * self[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * self[e321]), (other.group0()[1] * self[e321]), (other.group0()[2] * self[e321]), 0.0]),
        );
    }
}
impl GeometricProduct<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]])),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
    }
}
impl GeometricProduct<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       24        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[e321]) * Simd32x2::from([other.group4()[3], other.group1()[3]]) * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]])),
            // e41, e42, e43
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]])),
        );
    }
}
impl GeometricProduct<Origin> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e321] * other[e4] * -1.0));
    }
}
impl GeometricProduct<Plane> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[0] * self[e321] * -1.0),
                (other.group0()[1] * self[e321] * -1.0),
                (other.group0()[2] * self[e321] * -1.0),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e321] * -1.0)]),
        );
    }
}
impl GeometricProduct<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e321] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] * self[e321] * -1.0),
                (other.group0()[1] * self[e321] * -1.0),
                (other.group0()[2] * self[e321] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self[e321] * other[scalar]));
    }
}
impl InfixGeometricProduct for Line {}
impl GeometricProduct<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ (Simd32x3::from(other[e1234]) * self.group1()), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl GeometricProduct<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1())),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group1()),
        );
    }
}
impl GeometricProduct<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[0] * other.group1()[3]) - (self.group1()[1] * other.group0()[2])),
                ((self.group1()[1] * other.group1()[3]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[1]) + (self.group1()[2] * other.group1()[3])),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])
                    - (self.group1()[1] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[1])),
                (-(self.group0()[1] * other.group1()[3])
                    + (self.group0()[2] * other.group0()[0])
                    + (self.group1()[0] * other.group1()[2])
                    + (self.group1()[1] * other.group0()[3])
                    - (self.group1()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[2] * other.group1()[3]) - (self.group1()[0] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[0])
                    + (self.group1()[2] * other.group0()[3])),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group1()[0] * other[e321]), (self.group1()[1] * other[e321]), (self.group1()[2] * other[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other[e321] * -1.0),
                (self.group0()[1] * other[e321] * -1.0),
                (self.group0()[2] * other[e321] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       27        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) - (other.group0()[2] * self.group1()[1]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group0()[0] * self.group1()[2]) + (other.group0()[2] * self.group1()[0]) - (other.group1()[0] * self.group0()[2])
                    + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) - (other.group0()[1] * self.group1()[0]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
                (-(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) - (other.group1()[2] * self.group1()[1])),
                (-(other.group1()[0] * self.group1()[2]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) - (other.group1()[1] * self.group1()[0])),
                (-(other.group1()[0] * self.group1()[0]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]),
        );
    }
}
impl GeometricProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[2] * other.group1()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group1()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group1()[2]) + (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group0()[2]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[1] * other.group1()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3])),
                (-(self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(other.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((self.group1()[0] * other.group1()[3]) + (self.group1()[2] * other.group1()[1])),
                ((self.group1()[0] * other.group1()[2]) + (self.group1()[1] * other.group1()[3])),
                ((self.group1()[1] * other.group1()[0]) + (self.group1()[2] * other.group1()[3])),
                (-(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[0]]) * swizzle!(other.group1(), 2, 0, 1, 0))),
        );
    }
}
impl GeometricProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       57       72        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2])),
            ]) - (Simd32x2::from(other.group3()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(other.group3()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(other.group3()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[0] * other.group4()[3]) - (self.group1()[1] * other.group1()[2])),
                ((self.group1()[1] * other.group4()[3]) - (self.group1()[2] * other.group1()[0])),
                (-(self.group1()[0] * other.group1()[1]) + (self.group1()[2] * other.group4()[3])),
                ((self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1())
                - (swizzle!(self.group0(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group0(), 2, 0, 1) * swizzle!(other.group3(), 1, 2, 0))
                - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                + (swizzle!(self.group1(), 2, 0, 1) * swizzle!(other.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * self.group1()) - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group1(), 2, 0, 1) * swizzle!(other.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group4()[3]) + (self.group0()[1] * other.group1()[2]) + (self.group1()[0] * other.group1()[3])
                    - (self.group1()[1] * other.group4()[2])
                    + (self.group1()[2] * other.group4()[1])),
                (-(self.group0()[1] * other.group4()[3])
                    + (self.group0()[2] * other.group1()[0])
                    + (self.group1()[0] * other.group4()[2])
                    + (self.group1()[1] * other.group1()[3])
                    - (self.group1()[2] * other.group4()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[2] * other.group4()[3]) - (self.group1()[0] * other.group4()[1])
                    + (self.group1()[1] * other.group4()[0])
                    + (self.group1()[2] * other.group1()[3])),
                (-(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (-(self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       13       24        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) * -1.0),
                ((self.group1()[2] * other.group0()[0]) * -1.0),
                ((self.group1()[0] * other.group0()[1]) * -1.0),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])),
                ((self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3])),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1()),
        );
    }
}
impl InfixGeometricProduct for Motor {}
impl GeometricProduct<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e1234]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(other.group0()[0]) * self.group0()) + (Simd32x4::from(other.group0()[1]) * self.group1())),
            // e23, e31, e12, scalar
            (Simd32x4::from(other.group0()[0]) * self.group1()),
        );
    }
}
impl GeometricProduct<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       24        0
    //  no simd       44       48        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((other.group0()[2] * self.group0()[2]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]) - (other.group1()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]) * self.group1())
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[3], self.group0()[1]]) * swizzle!(other.group0(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group1()[2]) - (other.group1()[3] * self.group0()[0])),
                ((other.group0()[0] * self.group0()[2]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0]) - (other.group1()[3] * self.group0()[1])),
                ((other.group0()[1] * self.group0()[0]) + (other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3]) - (other.group1()[3] * self.group0()[2])),
                0.0,
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[3]]) * self.group1())
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]) * swizzle!(self.group1(), 1, 2, 0, 2))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[3], self.group1()[1]]) * swizzle!(other.group0(), 1, 2, 2, 1))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 0, 1, 0, 0))),
        );
    }
}
impl GeometricProduct<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
        );
    }
}
impl GeometricProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group0()[1] * self.group1()[3]) + (other.group0()[2] * self.group1()[0]) + (other.group1()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group1()[2])),
                ((other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       40       48        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) + (other.group0()[3] * self.group1()[0]) + (other.group1()[1] * self.group0()[2]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[2] * self.group1()[0]) + (other.group0()[3] * self.group1()[1]) + (other.group1()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group1()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[2] * self.group0()[3]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 0, 1, 0, 3) * swizzle!(self.group1(), 3, 3, 1, 3))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 0, 1, 0, 3) * swizzle!(self.group0(), 3, 3, 1, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) + (other.group1()[3] * self.group1()[0])),
                ((other.group1()[2] * self.group1()[0]) + (other.group1()[3] * self.group1()[1])),
                ((other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) + (swizzle!(other.group1(), 0, 1, 0, 3) * swizzle!(self.group1(), 3, 3, 1, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       37       47        0
    //  no simd       82       96        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((other.group0()[1] * self.group1()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) + (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (self.group1()[3] * other.group1()[0]),
                (self.group1()[1] * other.group4()[3]),
                (self.group1()[2] * other.group4()[3]),
                ((self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2])
                    - (self.group0()[3] * other.group4()[3])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[3], self.group1()[3], self.group0()[0]]) * swizzle!(other.group1(), 1, 1, 2, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group4()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group4()[3], other.group1()[2], other.group1()[0], other.group1()[3]]) * swizzle!(self.group1(), 0, 0, 1, 3))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group3(), 2, 0, 1))
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[3]]) * swizzle!(other.group3(), 1, 2, 2))
                + (Simd32x3::from([self.group0()[3], self.group0()[3], self.group0()[1]]) * swizzle!(other.group3(), 0, 1, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(other.group2(), 2, 0, 1))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(other.group2(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[3], self.group1()[3], self.group1()[1]]) * swizzle!(other.group2(), 0, 1, 0))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(other.group3(), 2, 0, 1))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(other.group3(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[3], self.group1()[3], self.group1()[1]]) * swizzle!(other.group3(), 0, 1, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group4()[3])
                    + (self.group0()[1] * other.group1()[2])
                    + (self.group1()[2] * other.group4()[1])
                    + (self.group1()[3] * other.group4()[0])),
                (-(self.group0()[1] * other.group4()[3])
                    + (self.group0()[2] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[3] * other.group4()[1])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[2] * other.group4()[3]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group4()[2])),
                0.0,
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(other.group1(), 0, 1, 2, 1))
                + (Simd32x4::from([other.group1()[3], other.group4()[2], other.group4()[0], other.group4()[3]]) * swizzle!(self.group1(), 0, 0, 1, 3))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group1()[2]]) * swizzle!(self.group1(), 1, 2, 0, 2))),
        );
    }
}
impl GeometricProduct<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group0()[3] * other.group0()[3])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (-(self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])
                    + (self.group1()[2] * other.group0()[1])
                    + (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])
                    + (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])
                    + (self.group1()[1] * other.group0()[0])
                    + (self.group1()[3] * other.group0()[2])),
                (self.group1()[3] * other.group0()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       20       32        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) * -1.0),
                ((self.group1()[2] * other.group0()[0]) * -1.0),
                ((self.group1()[0] * other.group0()[1]) * -1.0),
                ((self.group0()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(other.group0(), 0, 1, 2, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])),
                ((self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3])),
                ((self.group1()[2] * other.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(other.group0(), 0, 1, 2, 1))),
        );
    }
}
impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group1()),
        );
    }
}
impl InfixGeometricProduct for MultiVector {}
impl GeometricProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group0()[0] * other[e1234])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group4()[3] * other[e1234])]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e1234]), (self.group1()[1] * other[e1234]), (self.group1()[2] * other[e1234]), 0.0]),
        );
    }
}
impl GeometricProduct<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group0()[0] * self.group0()[0]),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group1()[0]),
                (other.group0()[0] * self.group1()[1]),
                (other.group0()[0] * self.group1()[2]),
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group4()[3])),
            ]),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group2()) + (Simd32x3::from(other.group0()[1]) * self.group3())),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group4()[0]) + (other.group0()[1] * self.group1()[0])),
                ((other.group0()[0] * self.group4()[1]) + (other.group0()[1] * self.group1()[1])),
                ((other.group0()[0] * self.group4()[2]) + (other.group0()[1] * self.group1()[2])),
                (other.group0()[0] * self.group4()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       51        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       81       97        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(other.group0()[0] * self.group4()[0]) - (other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2])
                    + (other.group1()[3] * self.group1()[3])),
            ]) + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]))
                - (Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group3()[0] * other.group1()[3]) - (self.group3()[1] * other.group0()[2])),
                ((self.group3()[1] * other.group1()[3]) - (self.group3()[2] * other.group0()[0])),
                (-(self.group3()[0] * other.group0()[1]) + (self.group3()[2] * other.group1()[3])),
                (-(self.group0()[1] * other.group1()[3]) + (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2])
                    - (self.group3()[0] * other.group1()[0])
                    - (self.group3()[1] * other.group1()[1])
                    - (self.group3()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group0())
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group0()[1] * self.group4()[2]) - (other.group0()[2] * self.group4()[1]) - (other.group1()[1] * self.group1()[2]) + (other.group1()[2] * self.group1()[1])),
                (-(other.group0()[0] * self.group4()[2]) + (other.group0()[2] * self.group4()[0]) + (other.group1()[0] * self.group1()[2])
                    - (other.group1()[2] * self.group1()[0])),
                ((other.group0()[0] * self.group4()[1]) - (other.group0()[1] * self.group4()[0]) - (other.group1()[0] * self.group1()[1]) + (other.group1()[1] * self.group1()[0])),
            ]) - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group0()[1] * self.group1()[2]) + (other.group0()[2] * self.group1()[1])),
                ((other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0])),
                (-(other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group1()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group2()[0] * other.group1()[3]) + (self.group2()[1] * other.group0()[2]) + (self.group3()[0] * other.group0()[3])
                    - (self.group3()[1] * other.group1()[2])
                    + (self.group3()[2] * other.group1()[1])),
                (-(self.group2()[1] * other.group1()[3])
                    + (self.group2()[2] * other.group0()[0])
                    + (self.group3()[0] * other.group1()[2])
                    + (self.group3()[1] * other.group0()[3])
                    - (self.group3()[2] * other.group1()[0])),
                ((self.group2()[0] * other.group0()[1]) - (self.group2()[2] * other.group1()[3]) - (self.group3()[0] * other.group1()[1])
                    + (self.group3()[1] * other.group1()[0])
                    + (self.group3()[2] * other.group0()[3])),
                ((self.group3()[2] * other.group0()[2]) * -1.0),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group1())
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group0(), 1, 2, 0, 1))),
        );
    }
}
impl GeometricProduct<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       29        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[e321]) * Simd32x2::from([self.group4()[3], self.group1()[3]]) * Simd32x2::from([-1.0, 1.0])),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]])),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
        );
    }
}
impl GeometricProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       57       72        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(other.group1()[0] * self.group2()[0]) - (other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2])),
            ]) - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[0] * self.group4()[3]) + (other.group1()[1] * self.group1()[2])),
                ((other.group1()[1] * self.group4()[3]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group4()[3])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group0())
                + (Simd32x3::from(self.group0()[1]) * other.group1())
                + (swizzle!(other.group0(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group0(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                + (swizzle!(other.group1(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                - (swizzle!(other.group1(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * other.group1()) + (swizzle!(other.group1(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group1(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group4()[3]) + (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group4()[2])
                    - (other.group1()[2] * self.group4()[1])),
                ((other.group0()[1] * self.group4()[3]) + (other.group0()[2] * self.group1()[0]) - (other.group1()[0] * self.group4()[2])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[2] * self.group4()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[2] * self.group4()[3]) + (other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0])
                    + (other.group1()[2] * self.group1()[3])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       33        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       81       97        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((self.group0()[1] * other.group1()[3]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2])),
            ]) + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))
                - (Simd32x2::from(other.group1()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group1()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group1()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) + (other.group1()[3] * self.group1()[0])),
                ((other.group1()[2] * self.group1()[0]) + (other.group1()[3] * self.group1()[1])),
                ((other.group1()[2] * self.group4()[3]) + (other.group1()[3] * self.group1()[2])),
                (-(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) + (other.group0()[3] * self.group4()[3])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group4()[3], self.group4()[3], self.group1()[1], self.group1()[3]]) * swizzle!(other.group1(), 0, 1, 0, 3))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([other.group0()[1], other.group0()[3], other.group0()[3]]) * swizzle!(self.group3(), 2, 1, 2))
                - (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * swizzle!(self.group3(), 1, 2, 0))
                + (Simd32x3::from([other.group0()[3], other.group0()[2], other.group0()[0]]) * swizzle!(self.group3(), 0, 0, 1))
                + (Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]) * swizzle!(self.group2(), 2, 1, 2))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * swizzle!(self.group2(), 1, 2, 0))
                + (Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]) * swizzle!(self.group2(), 0, 0, 1))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]) * swizzle!(self.group3(), 2, 1, 2))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * swizzle!(self.group3(), 1, 2, 0))
                + (Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]) * swizzle!(self.group3(), 0, 0, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2])
                    + (other.group0()[3] * self.group1()[0])
                    + (other.group1()[0] * self.group1()[3])
                    + (other.group1()[1] * self.group4()[2])
                    + (other.group1()[3] * self.group4()[0])),
                ((other.group0()[2] * self.group1()[0])
                    + (other.group0()[3] * self.group1()[1])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[2] * self.group4()[0])
                    + (other.group1()[3] * self.group4()[1])),
                ((other.group0()[0] * self.group1()[1])
                    + (other.group0()[3] * self.group1()[2])
                    + (other.group1()[0] * self.group4()[1])
                    + (other.group1()[2] * self.group1()[3])
                    + (other.group1()[3] * self.group4()[2])),
                ((other.group1()[2] * self.group1()[2]) * -1.0),
            ]) + (Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self.group1()[1]]) * swizzle!(other.group1(), 2, 0, 1, 1))),
        );
    }
}
impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       82        0
    //    simd2        8        8        0
    //    simd3       18       18        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      103      118        0
    //  no simd      177      192        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((other.group0()[1] * self.group0()[0])
                    - (other.group3()[0] * self.group2()[0])
                    - (other.group3()[1] * self.group2()[1])
                    - (other.group3()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])
                    + (other.group4()[3] * self.group1()[3])),
            ]) + (Simd32x2::from(other.group0()[0]) * self.group0())
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                - (Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group3()[1] * self.group1()[2]) + (self.group3()[0] * other.group4()[3]) - (self.group3()[1] * other.group1()[2])),
                ((other.group3()[2] * self.group1()[0]) + (self.group3()[1] * other.group4()[3]) - (self.group3()[2] * other.group1()[0])),
                ((other.group3()[0] * self.group1()[1]) - (self.group3()[0] * other.group1()[1]) + (self.group3()[2] * other.group4()[3])),
                (-(self.group0()[1] * other.group4()[3])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (other.group3()[0] * self.group4()[0])
                    - (other.group3()[1] * self.group4()[1])
                    - (other.group3()[2] * self.group4()[2])
                    + (self.group2()[1] * other.group1()[1])
                    + (self.group2()[2] * other.group1()[2])
                    - (self.group3()[0] * other.group4()[0])
                    - (self.group3()[1] * other.group4()[1])
                    - (self.group3()[2] * other.group4()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                + (Simd32x4::from(self.group0()[0]) * other.group1())
                + (Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group1()[1] * self.group4()[2]) - (other.group1()[2] * self.group4()[1]) - (other.group4()[1] * self.group1()[2]) + (other.group4()[2] * self.group1()[1])),
                (-(other.group1()[0] * self.group4()[2]) + (other.group1()[2] * self.group4()[0]) + (other.group4()[0] * self.group1()[2])
                    - (other.group4()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0]) - (other.group4()[0] * self.group1()[1]) + (other.group4()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group2())
                + (Simd32x3::from(other.group0()[1]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group3())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                + (swizzle!(other.group2(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group2(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                + (swizzle!(other.group3(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                - (swizzle!(other.group3(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group1()[2]) + (other.group1()[2] * self.group1()[1])),
                ((other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0])),
                (-(other.group1()[0] * self.group1()[1]) + (other.group1()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group3())
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group3(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group3(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[0])
                    + (other.group2()[0] * self.group4()[3])
                    + (other.group2()[1] * self.group1()[2])
                    + (other.group3()[0] * self.group1()[3])
                    + (other.group3()[1] * self.group4()[2])
                    - (other.group3()[2] * self.group4()[1])
                    - (self.group2()[0] * other.group4()[3])
                    + (self.group2()[1] * other.group1()[2])
                    + (self.group3()[0] * other.group1()[3])
                    - (self.group3()[1] * other.group4()[2])
                    + (self.group3()[2] * other.group4()[1])),
                ((other.group0()[1] * self.group1()[1]) + (other.group2()[1] * self.group4()[3]) + (other.group2()[2] * self.group1()[0]) - (other.group3()[0] * self.group4()[2])
                    + (other.group3()[1] * self.group1()[3])
                    + (other.group3()[2] * self.group4()[0])
                    - (self.group2()[1] * other.group4()[3])
                    + (self.group2()[2] * other.group1()[0])
                    + (self.group3()[0] * other.group4()[2])
                    + (self.group3()[1] * other.group1()[3])
                    - (self.group3()[2] * other.group4()[0])),
                ((other.group0()[1] * self.group1()[2]) + (other.group2()[0] * self.group1()[1]) + (other.group2()[2] * self.group4()[3]) + (other.group3()[0] * self.group4()[1])
                    - (other.group3()[1] * self.group4()[0])
                    + (other.group3()[2] * self.group1()[3])
                    + (self.group2()[0] * other.group1()[1])
                    - (self.group2()[2] * other.group4()[3])
                    - (self.group3()[0] * other.group4()[1])
                    + (self.group3()[1] * other.group4()[0])
                    + (self.group3()[2] * other.group1()[3])),
                (-(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]) - (self.group3()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group4())
                + (Simd32x4::from(self.group0()[0]) * other.group4())
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group1(), 1, 2, 0, 1))),
        );
    }
}
impl GeometricProduct<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group4()[3] * other[e4] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e4])]),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group3()[0] * other[e4]), (self.group3()[1] * other[e4]), (self.group3()[2] * other[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       44        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group4()[3] * other.group0()[3] * -1.0),
                ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] * other.group0()[3]),
                (self.group3()[1] * other.group0()[3]),
                (self.group3()[2] * other.group0()[3]),
                (-(self.group0()[1] * other.group0()[3])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[2] * other.group0()[2])),
            ]),
            // e41, e42, e43
            (Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[0]) - (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group0()[2]) + (self.group3()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[1]) - (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[2]) - (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group0()[1]) + (self.group3()[1] * other.group0()[0])),
                (self.group0()[0] * other.group0()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       35        0
    //    simd3        3        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       42        0
    //  no simd       40       60        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2])),
                (-(self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[3] * other.group0()[3])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group3()[1] * other.group0()[2]) * -1.0),
                ((self.group3()[2] * other.group0()[0]) * -1.0),
                ((self.group3()[0] * other.group0()[1]) * -1.0),
                ((self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2])),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group0())
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(self.group4()[1] * other.group0()[2]) + (self.group4()[2] * other.group0()[1])),
                ((self.group4()[0] * other.group0()[2]) - (self.group4()[2] * other.group0()[0])),
                (-(self.group4()[0] * other.group0()[1]) + (self.group4()[1] * other.group0()[0])),
            ]) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group2()[1] * other.group0()[2]) + (self.group3()[0] * other.group0()[3])),
                ((self.group2()[2] * other.group0()[0]) + (self.group3()[1] * other.group0()[3])),
                ((self.group2()[0] * other.group0()[1]) + (self.group3()[2] * other.group0()[3])),
                ((self.group3()[2] * other.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group0(), 1, 2, 0, 1))),
        );
    }
}
impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[scalar]) * self.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group4()),
        );
    }
}
impl InfixGeometricProduct for Origin {}
impl GeometricProduct<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (other.group0()[0] * self[e4]));
    }
}
impl GeometricProduct<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other[e321] * self[e4]));
    }
}
impl GeometricProduct<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(other.group1()[0] * self[e4]), (other.group1()[1] * self[e4]), (other.group1()[2] * self[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * self[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group1()[0] * self[e4]), (other.group1()[1] * self[e4]), (other.group1()[2] * self[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other.group4()[3] * self[e4])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self[e4])]),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group3()[0] * self[e4]), (other.group3()[1] * self[e4]), (other.group3()[2] * self[e4]), 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other.group0()[3] * self[e4]));
    }
}
impl GeometricProduct<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl GeometricProduct<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e4] * other[scalar]));
    }
}
impl InfixGeometricProduct for Plane {}
impl GeometricProduct<AntiScalar> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self.group0()[3] * other[e1234]));
    }
}
impl GeometricProduct<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self.group0()[3])]),
            // e423, e431, e412, e321
            (Simd32x4::from(other.group0()[0]) * self.group0()),
        );
    }
}
impl GeometricProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[1]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[1]]) * swizzle!(self.group0(), 3, 3, 3, 1))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl GeometricProduct<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * other[e321]), (self.group0()[1] * other[e321]), (self.group0()[2] * other[e321]), 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e321] * -1.0)]),
        );
    }
}
impl GeometricProduct<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]),
                (other.group1()[1] * self.group0()[3]),
                (other.group1()[2] * self.group0()[3]),
                (-(other.group1()[0] * self.group0()[0]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                ((other.group0()[1] * self.group0()[3]) - (other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]),
                (other.group1()[1] * self.group0()[3]),
                (other.group1()[2] * self.group0()[3]),
                ((other.group0()[3] * self.group0()[3]) - (other.group1()[0] * self.group0()[0]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) - (other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0]) + (other.group1()[3] * self.group0()[2])),
                (other.group1()[3] * self.group0()[3]),
            ]),
        );
    }
}
impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       44        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group4()[3] * self.group0()[3] * -1.0),
                (-(other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])
                    - (other.group1()[3] * self.group0()[3])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group3()[0] * self.group0()[3]),
                (other.group3()[1] * self.group0()[3]),
                (other.group3()[2] * self.group0()[3]),
                ((other.group0()[1] * self.group0()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0])),
                ((other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
            ]) + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[0]) + (other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group0()[2]) - (other.group3()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[1]) + (other.group2()[1] * self.group0()[3]) - (other.group3()[0] * self.group0()[2]) + (other.group3()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group0()[2]) + (other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group0()[1]) - (other.group3()[1] * self.group0()[0])),
                (other.group0()[0] * self.group0()[3]),
            ]),
        );
    }
}
impl GeometricProduct<Origin> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e4] * -1.0));
    }
}
impl GeometricProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group0()[0])),
                (-(other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1])),
                (-(other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self.group0()[3] * -1.0)]),
        );
    }
}
impl GeometricProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       16        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (self.group0()[2] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                (self.group0()[1] * other.group0()[0]),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group0()[3])),
            ]) - (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0] * -1.0),
                (self.group0()[3] * other.group0()[1] * -1.0),
                (self.group0()[3] * other.group0()[2] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(other[scalar]) * self.group0()));
    }
}
impl InfixGeometricProduct for Point {}
impl GeometricProduct<AntiScalar> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([
            (self.group0()[0] * other[e1234]),
            (self.group0()[1] * other[e1234]),
            (self.group0()[2] * other[e1234]),
            0.0,
        ]));
    }
}
impl GeometricProduct<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other.group0()[0]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group0()[0]),
                (other.group0()[1] * self.group0()[1]),
                (other.group0()[1] * self.group0()[2]),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(other.group0()[3] * self.group0()[0]) - (other.group1()[1] * self.group0()[2])),
                (-(other.group0()[3] * self.group0()[1]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group0()[3] * self.group0()[2]) - (other.group1()[0] * self.group0()[1])),
                ((other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                + (swizzle!(other.group1(), 2, 0, 1, 1) * swizzle!(self.group0(), 1, 2, 0, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) - (other.group1()[3] * self.group0()[0])),
                (-(other.group0()[2] * self.group0()[0]) - (other.group1()[3] * self.group0()[1])),
                (-(other.group0()[0] * self.group0()[1]) - (other.group1()[3] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e321])]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] * other[e321] * -1.0),
                (self.group0()[1] * other[e321] * -1.0),
                (self.group0()[2] * other[e321] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       13       21        0
    fn geometric_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group1()[1] * self.group0()[2]),
                (other.group1()[2] * self.group0()[0]),
                (other.group1()[0] * self.group0()[1]),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3])),
                ((other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group1()[3] * self.group0()[0]),
                (other.group1()[3] * self.group0()[1]),
                (other.group1()[3] * self.group0()[2]),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group0()[3] * self.group0()[0]) + (other.group1()[0] * self.group0()[3])),
                ((other.group0()[2] * self.group0()[0]) + (other.group0()[3] * self.group0()[1]) + (other.group1()[1] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[3] * self.group0()[2]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       41       56        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([0.0, (other.group4()[3] * self.group0()[3])])
                + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group3()[1] * self.group0()[2]),
                (other.group3()[2] * self.group0()[0]),
                (other.group3()[0] * self.group0()[1]),
                (-(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group0())
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(other.group4()[1] * self.group0()[2]) + (other.group4()[2] * self.group0()[1])),
                ((other.group4()[0] * self.group0()[2]) - (other.group4()[2] * self.group0()[0])),
                (-(other.group4()[0] * self.group0()[1]) + (other.group4()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group0()[2]) + (other.group1()[2] * self.group0()[1])),
                ((other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group1()[0] * self.group0()[1]) + (other.group1()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[0]) + (other.group2()[1] * self.group0()[2]) + (other.group3()[0] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[1]) + (other.group2()[2] * self.group0()[0]) + (other.group3()[1] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[1]) + (other.group3()[2] * self.group0()[3])),
                (-(other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl GeometricProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        6       19        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self.group0()[3])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[3] * self.group0()[0] * -1.0),
                (other.group0()[3] * self.group0()[1] * -1.0),
                (other.group0()[3] * self.group0()[2] * -1.0),
                0.0,
            ]),
        );
    }
}
impl GeometricProduct<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        8       18        0
    fn geometric_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) - (other.group0()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) - (other.group0()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) - (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
    }
}
impl GeometricProduct<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(other[scalar]) * self.group0()));
    }
}
impl InfixGeometricProduct for Scalar {}
impl GeometricProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (other[e1234] * self[scalar]));
    }
}
impl GeometricProduct<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(self[scalar]) * other.group0()));
    }
}
impl GeometricProduct<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl GeometricProduct<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (other[e321] * self[scalar]));
    }
}
impl GeometricProduct<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1()),
        );
    }
}
impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[scalar]) * other.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group4()),
        );
    }
}
impl GeometricProduct<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (other[e4] * self[scalar]));
    }
}
impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl GeometricProduct<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[scalar] * self[scalar]));
    }
}
