// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl InfixWedge for AntiScalar {}
impl Wedge<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e1234] * other.group0()[0]));
    }
}
impl Wedge<Motor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e1234] * other.group1()[3]));
    }
}
impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e1234] * other.group0()[0]));
    }
}
impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e1234] * other[scalar]));
    }
}
impl InfixWedge for DualNum {}
impl Wedge<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[0] * other[e1234]));
    }
}
impl Wedge<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (self.group0()[0] * other.group0()[0]),
            ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
        ]));
    }
}
impl Wedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[0]) * other.group1()),
        );
    }
}
impl Wedge<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self.group0()[0] * other[e321]));
    }
}
impl Wedge<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group1()),
        );
    }
}
impl Wedge<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[3])),
            ]),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[0]) * other.group1()),
        );
    }
}
impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group2()),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[0]) * other.group4()),
        );
    }
}
impl Wedge<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self.group0()[0] * other[e4]));
    }
}
impl Wedge<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self.group0()[0]) * other.group0()));
    }
}
impl Wedge<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Point) -> Self::Output {
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self.group0()[0]) * other.group0()));
    }
}
impl Wedge<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ (self.group0() * Simd32x2::from(other[scalar])));
    }
}
impl InfixWedge for Flector {}
impl Wedge<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e423, e431, e412, e321
            (self.group1() * Simd32x4::from(other.group0()[0])),
        );
    }
}
impl Wedge<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[0] * other.group0()[0])
                        + (self.group0()[2] * other.group1()[2])
                        + (self.group0()[0] * other.group1()[0])
                        + (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Horizon> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e321]));
    }
}
impl Wedge<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group1()[3])),
            // e423, e431, e412, e321
            ((self.group1() * Simd32x4::from(other.group1()[3]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       25       40        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    + (self.group0()[3] * other.group4()[3])
                    + (self.group0()[2] * other.group4()[2])
                    + (self.group0()[0] * other.group4()[0])
                    + (self.group0()[1] * other.group4()[1])),
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            (-(Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
            ]),
            // e423, e431, e412, e321
            ((self.group1() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group3()[0]) + (self.group0()[2] * other.group2()[1])),
                    ((self.group0()[3] * other.group3()[1]) + (self.group0()[0] * other.group2()[2])),
                    ((self.group0()[3] * other.group3()[2]) + (self.group0()[1] * other.group2()[0])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl Wedge<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl Wedge<Plane> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e1234
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn wedge(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    (-(self.group1()[2] * other.group0()[2]) - (self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other[scalar])),
            // e423, e431, e412, e321
            (self.group1() * Simd32x4::from(other[scalar])),
        );
    }
}
impl InfixWedge for Horizon {}
impl Wedge<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self[e321] * other.group0()[0]));
    }
}
impl Wedge<Flector> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e321] * other.group0()[3] * -1.0));
    }
}
impl Wedge<Motor> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self[e321] * other.group1()[3]));
    }
}
impl Wedge<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e321] * other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self[e321] * other.group0()[0])]),
        );
    }
}
impl Wedge<Origin> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e321] * other[e4] * -1.0));
    }
}
impl Wedge<Point> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e321] * other.group0()[3] * -1.0));
    }
}
impl Wedge<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self[e321] * other[scalar]));
    }
}
impl InfixWedge for Line {}
impl Wedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(other.group0()[0])),
        );
    }
}
impl Wedge<Flector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Plane::from_groups(
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Line> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Line) -> Self::Output {
        return AntiScalar::from_groups(
            // e1234
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]),
                (self.group0()[1] * other.group1()[3]),
                (self.group0()[2] * other.group1()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group1()[0] * other.group1()[3]),
                (self.group1()[1] * other.group1()[3]),
                (self.group1()[2] * other.group1()[3]),
                0.0,
            ]),
        );
    }
}
impl Wedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group1()[2] * other.group2()[2])
                    - (self.group1()[1] * other.group2()[1])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(other.group0()[0])),
            // e423, e431, e412, e321
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group1()[3]) + (self.group0()[1] * other.group1()[2])),
                    ((self.group1()[1] * other.group1()[3]) + (self.group0()[2] * other.group1()[0])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group0()[0] * other.group1()[1])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
    }
}
impl Wedge<Point> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Point) -> Self::Output {
        return Plane::from_groups(
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(other[scalar])),
        );
    }
}
impl InfixWedge for Motor {}
impl Wedge<AntiScalar> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group1()[3] * other[e1234]));
    }
}
impl Wedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[2] * other.group0()[0]),
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[3] * other.group0()[1])),
            ]),
            // e23, e31, e12, scalar
            (self.group1() * Simd32x4::from(other.group0()[0])),
        );
    }
}
impl Wedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group1()[3]) * other.group0()),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * other.group1())
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Horizon> for Motor {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self.group1()[3] * other[e321]));
    }
}
impl Wedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group1()[3] * other.group0()[0]),
                (self.group1()[3] * other.group0()[1]),
                (self.group1()[3] * other.group0()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group1()[3] * other.group1()[0]),
                (self.group1()[3] * other.group1()[1]),
                (self.group1()[3] * other.group1()[2]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group0() * Simd32x4::from(other.group1()[3]))
                + (Simd32x4::from(self.group1()[3]) * other.group0())
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0])),
                ((self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1])),
                ((self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2])),
                (self.group1()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       25       41        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group1()[3] * other.group0()[0]),
                ((self.group1()[3] * other.group0()[1]) - (self.group1()[2] * other.group2()[2]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[0] * other.group2()[0])
                    + (self.group0()[3] * other.group0()[0])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group1()[3]) * other.group1()),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group1()[3]) * other.group2())),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])) + (Simd32x3::from(self.group1()[3]) * other.group3())),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * other.group4())
                - (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group1()[3]) + (self.group0()[1] * other.group1()[2])),
                    ((self.group1()[1] * other.group1()[3]) + (self.group0()[2] * other.group1()[0])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group0()[0] * other.group1()[1])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
    }
}
impl Wedge<Plane> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self.group1()[3]) * other.group0()));
    }
}
impl Wedge<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group1()[3]) * other.group0()),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0() * Simd32x4::from(other[scalar])),
            // e23, e31, e12, scalar
            (self.group1() * Simd32x4::from(other[scalar])),
        );
    }
}
impl InfixWedge for MultiVector {}
impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[0] * other[e1234]));
    }
}
impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            (self.group2() * Simd32x3::from(other.group0()[0])),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(other.group0()[0])),
            // e423, e431, e412, e321
            (self.group4() * Simd32x4::from(other.group0()[0])),
        );
    }
}
impl Wedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       25       40        0
    fn wedge(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group4()[3] * other.group0()[3])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[1] * other.group0()[1])
                    - (self.group4()[0] * other.group0()[0])
                    + (self.group1()[3] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[2])
                    + (self.group1()[0] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group1())
                + Simd32x4::from([
                    ((self.group3()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                    ((self.group3()[1] * other.group0()[3]) + (self.group2()[2] * other.group0()[0])),
                    ((self.group3()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                    (-(self.group3()[1] * other.group0()[1]) - (self.group3()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl Wedge<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group1()[3] * other[e321])]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e321])]),
        );
    }
}
impl Wedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn wedge(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group3()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group1()),
            // e423, e431, e412, e321
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group0()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group0()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group0()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       25       41        0
    fn wedge(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group1()[3]),
                (-(self.group3()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    + (self.group0()[0] * other.group0()[3])
                    + (self.group0()[1] * other.group1()[3])),
            ]),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other.group1()[3])),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])) + (self.group2() * Simd32x3::from(other.group1()[3]))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])) + (self.group3() * Simd32x3::from(other.group1()[3]))),
            // e423, e431, e412, e321
            ((self.group4() * Simd32x4::from(other.group1()[3]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group0()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group0()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group0()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        5        6        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       40       51        0
    //  no simd       65       81        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                (-(self.group4()[3] * other.group1()[3])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group4()[1] * other.group1()[1])
                    - (self.group4()[0] * other.group1()[0])
                    - (self.group3()[2] * other.group2()[2])
                    - (self.group3()[1] * other.group2()[1])
                    - (self.group3()[0] * other.group2()[0])
                    - (self.group2()[2] * other.group3()[2])
                    - (self.group2()[1] * other.group3()[1])
                    - (self.group2()[0] * other.group3()[0])
                    + (self.group1()[3] * other.group4()[3])
                    + (self.group1()[2] * other.group4()[2])
                    + (self.group1()[1] * other.group4()[1])
                    + (self.group1()[0] * other.group4()[0])
                    + (self.group0()[0] * other.group0()[1])
                    + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group0()[0]) * other.group1()) + (self.group1() * Simd32x4::from(other.group0()[0]))),
            // e41, e42, e43
            ((self.group2() * Simd32x3::from(other.group0()[0]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            ((self.group3() * Simd32x3::from(other.group0()[0]))
                + (Simd32x3::from(self.group0()[0]) * other.group3())
                + Simd32x3::from([
                    (-(self.group1()[2] * other.group1()[1]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[1] * other.group1()[0]) + (self.group1()[0] * other.group1()[1])),
                ])),
            // e423, e431, e412, e321
            ((self.group4() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group4())
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group3()[0] * other.group1()[3])
                        + (self.group2()[1] * other.group1()[2])
                        + (self.group1()[3] * other.group3()[0])
                        + (self.group1()[2] * other.group2()[1])),
                    ((self.group3()[1] * other.group1()[3])
                        + (self.group2()[2] * other.group1()[0])
                        + (self.group1()[3] * other.group3()[1])
                        + (self.group1()[0] * other.group2()[2])),
                    ((self.group3()[2] * other.group1()[3])
                        + (self.group2()[0] * other.group1()[1])
                        + (self.group1()[3] * other.group3()[2])
                        + (self.group1()[1] * other.group2()[0])),
                    (-(self.group3()[1] * other.group1()[1])
                        - (self.group3()[0] * other.group1()[0])
                        - (self.group1()[1] * other.group3()[1])
                        - (self.group1()[0] * other.group3()[0])),
                ])),
        );
    }
}
impl Wedge<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn wedge(self, other: Origin) -> Self::Output {
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
impl Wedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                ((self.group1()[3] * other.group0()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[0]) * other.group0()),
        );
    }
}
impl Wedge<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       17       32        0
    fn wedge(self, other: Point) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group4()[3] * other.group0()[3])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                + Simd32x4::from([
                    ((self.group3()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                    ((self.group3()[1] * other.group0()[3]) + (self.group2()[2] * other.group0()[0])),
                    ((self.group3()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                    (-(self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            (self.group0() * Simd32x2::from(other[scalar])),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other[scalar])),
            // e41, e42, e43
            (self.group2() * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(other[scalar])),
            // e423, e431, e412, e321
            (self.group4() * Simd32x4::from(other[scalar])),
        );
    }
}
impl InfixWedge for Origin {}
impl Wedge<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e4] * other.group0()[0]));
    }
}
impl Wedge<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl Wedge<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e4] * other[e321]));
    }
}
impl Wedge<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group1()[0]), (self[e4] * other.group1()[1]), (self[e4] * other.group1()[2]), 0.0]),
        );
    }
}
impl Wedge<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4] * other.group1()[3])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group1()[0]), (self[e4] * other.group1()[1]), (self[e4] * other.group1()[2]), 0.0]),
        );
    }
}
impl Wedge<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e4] * other.group4()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4] * other.group0()[0])]),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group3()[0]), (self[e4] * other.group3()[1]), (self[e4] * other.group3()[2]), 0.0]),
        );
    }
}
impl Wedge<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[e4] * other.group0()[3]));
    }
}
impl Wedge<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[e4] * other[scalar]));
    }
}
impl InfixWedge for Plane {}
impl Wedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(other.group0()[0])));
    }
}
impl Wedge<Flector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        return AntiScalar::from_groups(
            // e1234
            (-(self.group0()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(other.group1()[3])));
    }
}
impl Wedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(other.group0()[0])),
        );
    }
}
impl Wedge<Origin> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e4] * -1.0));
    }
}
impl Wedge<Point> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Point) -> Self::Output {
        return AntiScalar::from_groups(
            // e1234
            (-(self.group0()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(other[scalar])));
    }
}
impl InfixWedge for Point {}
impl Wedge<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Point::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(other.group0()[0])));
    }
}
impl Wedge<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) * -1.0),
                    ((self.group0()[1] * other.group0()[3]) * -1.0),
                    ((self.group0()[2] * other.group0()[3]) * -1.0),
                    ((self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Horizon> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e321]));
    }
}
impl Wedge<Line> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group1()[3])),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       17       32        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                ((self.group0()[3] * other.group4()[3]) + (self.group0()[2] * other.group4()[2]) + (self.group0()[0] * other.group4()[0]) + (self.group0()[1] * other.group4()[1])),
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            (-(Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
            ]),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group3()[0]) + (self.group0()[2] * other.group2()[1])),
                    ((self.group0()[3] * other.group3()[1]) + (self.group0()[0] * other.group2()[2])),
                    ((self.group0()[3] * other.group3()[2]) + (self.group0()[1] * other.group2()[0])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl Wedge<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Plane> for Point {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e1234
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Point> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn wedge(self, other: Point) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12
            Simd32x3::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
            ]),
        );
    }
}
impl Wedge<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(other[scalar])));
    }
}
impl InfixWedge for Scalar {}
impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (self[scalar] * other[e1234]));
    }
}
impl Wedge<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ (self[scalar] * other[e321]));
    }
}
impl Wedge<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn wedge(self, other: MultiVector) -> Self::Output {
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
impl Wedge<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ (self[scalar] * other[e4]));
    }
}
impl Wedge<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other[scalar]));
    }
}
