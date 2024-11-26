// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 99
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       2       0
//  Average:         3       6       0
//  Maximum:        46      57       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       6       0
//  Average:         3       9       0
//  Maximum:        65      81       0
impl std::ops::Div<anti_wedge> for AntiScalar {
    type Output = anti_wedge_partial<AntiScalar>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234]);
    }
}
impl AntiWedge<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
    }
}
impl AntiWedge<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        );
    }
}
impl AntiWedge<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e4]);
    }
}
impl AntiWedge<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<anti_wedge> for DualNum {
    type Output = anti_wedge_partial<DualNum>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
        );
    }
}
impl AntiWedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
    }
}
impl AntiWedge<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e1234] * other[e23],
                self[e1234] * other[e31],
                self[e1234] * other[e12],
                (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        );
    }
}
impl AntiWedge<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e4]);
    }
}
impl AntiWedge<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<anti_wedge> for Flector {
    type Output = anti_wedge_partial<Flector>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) + (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]])),
        );
    }
}
impl AntiWedge<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]),
        );
    }
}
impl AntiWedge<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
            (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
            (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e423] * other[e31]),
            -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
        ]));
    }
}
impl AntiWedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
                (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
                (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e423] * other[e31]),
                -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
                    - (self[e423] * other[e1])
                    - (self[e431] * other[e2])
                    - (self[e412] * other[e3])
                    - (self[e321] * other[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]),
                (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]),
                (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e423] * other[e31]),
                -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
            ]),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<Origin> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0);
    }
}
impl AntiWedge<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e321] * other[e423]) * -1.0,
                (self[e321] * other[e431]) * -1.0,
                (self[e321] * other[e412]) * -1.0,
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
            ]) + (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]])),
        );
    }
}
impl AntiWedge<Point> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        );
    }
}
impl std::ops::Div<anti_wedge> for Horizon {
    type Output = anti_wedge_partial<Horizon>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e1234] * self[e321]);
    }
}
impl AntiWedge<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e1234] * self[e321]);
    }
}
impl AntiWedge<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl AntiWedge<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl AntiWedge<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e321] * other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl AntiWedge<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       21        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e321] * other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl AntiWedge<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0);
    }
}
impl AntiWedge<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
        );
    }
}
impl AntiWedge<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0);
    }
}
impl std::ops::Div<anti_wedge> for Line {
    type Output = anti_wedge_partial<Line>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<Flector> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (other[e412] * self[e31]) + (other[e321] * self[e41]) - (other[e431] * self[e12]),
            (other[e423] * self[e12]) + (other[e321] * self[e42]) - (other[e412] * self[e23]),
            (other[e431] * self[e23]) + (other[e321] * self[e43]) - (other[e423] * self[e31]),
            -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
        ]));
    }
}
impl AntiWedge<Horizon> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl AntiWedge<Line> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
        );
    }
}
impl AntiWedge<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e41] * other[e1234], self[e42] * other[e1234], self[e43] * other[e1234], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e23] * other[e1234],
                self[e31] * other[e1234],
                self[e12] * other[e1234],
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
            (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
            (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
            -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
        ]));
    }
}
impl std::ops::Div<anti_wedge> for Motor {
    type Output = anti_wedge_partial<Motor>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl AntiWedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e1234] * self[e23],
                other[e1234] * self[e31],
                other[e1234] * self[e12],
                (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]),
            ]),
        );
    }
}
impl AntiWedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e31]) + (other[e321] * self[e41]) - (other[e431] * self[e12]),
                (other[e423] * self[e12]) + (other[e321] * self[e42]) - (other[e412] * self[e23]),
                (other[e431] * self[e23]) + (other[e321] * self[e43]) - (other[e423] * self[e31]),
                -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321] * self[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl AntiWedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e41] * self[e1234], other[e42] * self[e1234], other[e43] * self[e1234], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e23] * self[e1234],
                other[e31] * self[e1234],
                other[e12] * self[e1234],
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
        );
    }
}
impl AntiWedge<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[e1234]) + (other[e1234] * self[e41]),
                (other[e42] * self[e1234]) + (other[e1234] * self[e42]),
                (other[e43] * self[e1234]) + (other[e1234] * self[e43]),
                other[e1234] * self[e1234],
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1()),
        );
    }
}
impl AntiWedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       25       41        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1234] * other[scalar]) + (self[scalar] * other[e1234])
                    - (self[e41] * other[e23])
                    - (self[e42] * other[e31])
                    - (self[e43] * other[e12])
                    - (self[e23] * other[e41])
                    - (self[e31] * other[e42])
                    - (self[e12] * other[e43]),
                self[e1234] * other[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group2()) + (Simd32x3::from(other[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]])),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group3()) + (Simd32x3::from(other[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        );
    }
}
impl AntiWedge<Origin> for Motor {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e4]);
    }
}
impl AntiWedge<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
    }
}
impl AntiWedge<Point> for Motor {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl AntiWedge<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<anti_wedge> for MultiVector {
    type Output = anti_wedge_partial<MultiVector>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
    }
}
impl AntiWedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
    }
}
impl AntiWedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e31]) + (other[e321] * self[e41]) - (other[e431] * self[e12]),
                (other[e423] * self[e12]) + (other[e321] * self[e42]) - (other[e412] * self[e23]),
                (other[e431] * self[e23]) + (other[e321] * self[e43]) - (other[e423] * self[e31]),
                -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl AntiWedge<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       18        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321] * self[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl AntiWedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       25       41        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) + (other[scalar] * self[e1234])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
                other[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e41], other[e42], other[e43]])),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e23], other[e31], other[e12]])),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
    }
}
impl AntiWedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       47        0
    //    simd3        5        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       46       57        0
    //  no simd       65       81        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * self[e1234])
                    + (other[e1234] * self[scalar])
                    + (other[e423] * self[e1])
                    + (other[e431] * self[e2])
                    + (other[e412] * self[e3])
                    + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321])
                    - (other[e41] * self[e23])
                    - (other[e42] * self[e31])
                    - (other[e43] * self[e12])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
                other[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) + (other[e412] * self[e31]) + (other[e321] * self[e41])
                    - (other[e12] * self[e431])
                    - (other[e431] * self[e12]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) + (other[e423] * self[e12]) + (other[e321] * self[e42])
                    - (other[e23] * self[e412])
                    - (other[e412] * self[e23]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e431] * self[e23]) + (other[e321] * self[e43])
                    - (other[e31] * self[e423])
                    - (other[e423] * self[e31]),
                -(other[e41] * self[e423])
                    - (other[e42] * self[e431])
                    - (other[e43] * self[e412])
                    - (other[e423] * self[e41])
                    - (other[e431] * self[e42])
                    - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]) + (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group2()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + (Simd32x3::from(self[e1234]) * other.group3())
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4()) + (Simd32x4::from(self[e1234]) * other.group4()),
        );
    }
}
impl AntiWedge<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, self[e1234] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       17       32        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
            ]),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
    }
}
impl AntiWedge<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<anti_wedge> for Origin {
    type Output = anti_wedge_partial<Origin>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
    }
}
impl AntiWedge<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
    }
}
impl AntiWedge<Flector> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
    }
}
impl AntiWedge<Horizon> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
    }
}
impl AntiWedge<Motor> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
    }
}
impl AntiWedge<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e4] * other[e321]);
    }
}
impl std::ops::Div<anti_wedge> for Plane {
    type Output = anti_wedge_partial<Plane>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e321] * self[e423],
                other[e321] * self[e431],
                other[e321] * self[e412],
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]])),
        );
    }
}
impl AntiWedge<Horizon> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
        );
    }
}
impl AntiWedge<Line> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
            (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
            (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
            -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
        ]));
    }
}
impl AntiWedge<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
    }
}
impl AntiWedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       17       32        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
    }
}
impl AntiWedge<Origin> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_wedge(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e4] * self[e321] * -1.0);
    }
}
impl AntiWedge<Plane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
        );
    }
}
impl AntiWedge<Point> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        );
    }
}
impl std::ops::Div<anti_wedge> for Point {
    type Output = anti_wedge_partial<Point>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<Flector> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        );
    }
}
impl AntiWedge<Horizon> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
    }
}
impl AntiWedge<Motor> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl AntiWedge<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        );
    }
}
impl std::ops::Div<anti_wedge> for Scalar {
    type Output = anti_wedge_partial<Scalar>;
    fn div(self, _rhs: anti_wedge) -> Self::Output {
        anti_wedge_partial(self)
    }
}
impl AntiWedge<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
impl AntiWedge<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
impl AntiWedge<Motor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
impl AntiWedge<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
