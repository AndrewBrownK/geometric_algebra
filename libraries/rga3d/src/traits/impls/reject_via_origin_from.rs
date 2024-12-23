// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 75
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       9       0
//  Average:        10      17       0
//  Maximum:        76      97       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         5      20       0
//  Average:        17      31       0
//  Maximum:       130     165       0
impl std::ops::Div<reject_via_origin_from> for AntiScalar {
    type Output = reject_via_origin_from_partial<AntiScalar>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for AntiScalar {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl RejectViaOriginFrom<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e2] * right_dual[e431]) + (anti_wedge[e3] * right_dual[e412]) + (anti_wedge[e4] * right_dual[e321])
                        - (anti_wedge[e431] * right_dual[e2])
                        - (anti_wedge[e412] * right_dual[e3])
                        - (anti_wedge[e321] * right_dual[e4]),
                )
                - (right_dual.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().yzx() * right_dual.group0().zxy()) - (anti_wedge.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for AntiScalar {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectViaOriginFrom<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge[e41] * right_dual[e23])
                - (anti_wedge[e42] * right_dual[e31])
                - (anti_wedge[e43] * right_dual[e12])
                - (anti_wedge[e23] * right_dual[e41])
                - (anti_wedge[e31] * right_dual[e42])
                - (anti_wedge[e12] * right_dual[e43]),
        );
    }
}
impl RejectViaOriginFrom<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       13       27        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group0())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * right_dual[e23])
                        - (anti_wedge[e42] * right_dual[e31])
                        - (anti_wedge[e43] * right_dual[e12])
                        - (anti_wedge[e23] * right_dual[e41])
                        - (anti_wedge[e31] * right_dual[e42])
                        - (anti_wedge[e12] * right_dual[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * right_dual[scalar]),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       35        0
    //    simd2        0        1        0
    //    simd3        6       12        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       65      101        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * right_dual[scalar],
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for AntiScalar {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectViaOriginFrom<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return AntiScalar::from_groups(
            // e1234
            (right_dual[e423] * anti_wedge[e1]) + (right_dual[e431] * anti_wedge[e2]) + (right_dual[e412] * anti_wedge[e3]) + (right_dual[e321] * anti_wedge[e4]),
        );
    }
}
impl RejectViaOriginFrom<Scalar> for AntiScalar {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<reject_via_origin_from> for DualNum {
    type Output = reject_via_origin_from_partial<DualNum>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (f32::powi(other[scalar], 2) * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectViaOriginFrom<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e2] * right_dual[e431]) + (anti_wedge[e3] * right_dual[e412]) + (anti_wedge[e4] * right_dual[e321])
                        - (anti_wedge[e431] * right_dual[e2])
                        - (anti_wedge[e412] * right_dual[e3])
                        - (anti_wedge[e321] * right_dual[e4]),
                )
                - (right_dual.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().yzx() * right_dual.group0().zxy()) - (anti_wedge.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for DualNum {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectViaOriginFrom<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge[e41] * right_dual[e23])
                - (anti_wedge[e42] * right_dual[e31])
                - (anti_wedge[e43] * right_dual[e12])
                - (anti_wedge[e23] * right_dual[e41])
                - (anti_wedge[e31] * right_dual[e42])
                - (anti_wedge[e12] * right_dual[e43]),
        );
    }
}
impl RejectViaOriginFrom<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       14       28        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e23], other[e31], other[e12], 1.0]) * self.group0().yy().with_zw(self[e1234], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group0())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * right_dual[e23])
                        - (anti_wedge[e42] * right_dual[e31])
                        - (anti_wedge[e43] * right_dual[e12])
                        - (anti_wedge[e23] * right_dual[e41])
                        - (anti_wedge[e31] * right_dual[e42])
                        - (anti_wedge[e12] * right_dual[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       37        0
    //    simd3        6       12        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       39       56        0
    //  no simd       66      101        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for DualNum {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[e321], 2));
    }
}
impl RejectViaOriginFrom<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return AntiScalar::from_groups(
            // e1234
            (right_dual[e423] * anti_wedge[e1]) + (right_dual[e431] * anti_wedge[e2]) + (right_dual[e412] * anti_wedge[e3]) + (right_dual[e321] * anti_wedge[e4]),
        );
    }
}
impl RejectViaOriginFrom<Scalar> for DualNum {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<reject_via_origin_from> for Flector {
    type Output = reject_via_origin_from_partial<Flector>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       28       41        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().yzx() * self.group1().zxy()) - (other.group1().zxy() * self.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (other.group1().wwwx() * self.group1().xyz().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                )
                - (self.group1().wwwx() * other.group1().xyz().with_w(other[e1])),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e4] * anti_wedge[e23]) + (right_dual[e423] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e31]) + (right_dual[e431] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e12]) + (right_dual[e412] * anti_wedge[scalar]),
                -(right_dual[e2] * anti_wedge[e31]) - (right_dual[e3] * anti_wedge[e12]),
            ]) + (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(right_dual[e321] * anti_wedge[scalar])
                - (right_dual.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]),
        );
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * right_dual[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0]) * anti_wedge.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       16       24        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e431] * other[e23]) + (self[e321] * other[e43]),
                -(self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * anti_wedge.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        5        4        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       24       36        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e431] * other[e23]) + (self[e321] * other[e43]),
                -(self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0())
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group1().xyz()).with_w(0.0)
                + (anti_wedge.group0().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (anti_wedge.group0().yzx() * right_dual.group0().zxy()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       50        0
    //    simd3        8       14        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       53       72        0
    //  no simd       90      124        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
                (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e431] * other[e23]) + (self[e321] * other[e43]),
                -(self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0())
                - (self.group1().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (self.group1().zxy() * other.group4().yzx()) - (self.group1().yzx() * other.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group1().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       11        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        9       29        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group1().zxy() * other.group0().yzx()) - (self.group1().yzx() * other.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e321] * other[e423] * -1.0,
                self[e321] * other[e431] * -1.0,
                self[e321] * other[e412] * -1.0,
                (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) + (other.group0().wwwx() * self.group1().xyz().with_w(self[e1])),
        );
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * right_dual[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0]) * anti_wedge.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Point> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(
            // scalar
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 0.0])
                * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]).xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl std::ops::Div<reject_via_origin_from> for Horizon {
    type Output = reject_via_origin_from_partial<Horizon>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       29        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e4] * anti_wedge[e23]) + (right_dual[e423] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e31]) + (right_dual[e431] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e12]) + (right_dual[e412] * anti_wedge[scalar]),
                -(right_dual[e2] * anti_wedge[e31]) - (right_dual[e3] * anti_wedge[e12]),
            ]) + (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(right_dual[e321] * anti_wedge[scalar])
                - (right_dual.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl RejectViaOriginFrom<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8       20        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * anti_wedge.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        3       11        0
    //  no simd       12       37        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * anti_wedge.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group1().xyz()).with_w(0.0)
                + (anti_wedge.group0().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (anti_wedge.group0().yzx() * right_dual.group0().zxy()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       37        0
    //    simd2        0        1        0
    //    simd3        6       12        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       38       57        0
    //  no simd       65      103        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group2().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * other[e1234]),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * right_dual[scalar],
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       15        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0])
                * Simd32x3::from([self[e321] * other[e423] * -1.0, self[e321] * other[e431] * -1.0, self[e321] * other[e412] * -1.0]).with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(self[e321] * other[e4] * -1.0) * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Div<reject_via_origin_from> for Line {
    type Output = reject_via_origin_from_partial<Line>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       17       32        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e431] * self[e23]) + (other[e321] * self[e43]),
                -(other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual[e4] * anti_wedge[e1] * -1.0,
                right_dual[e4] * anti_wedge[e2] * -1.0,
                right_dual[e4] * anti_wedge[e3] * -1.0,
                (right_dual[e431] * anti_wedge[e2]) + (right_dual[e412] * anti_wedge[e3]) + (right_dual[e321] * anti_wedge[e4]),
            ]) + (anti_wedge.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423])),
            // e23, e31, e12, scalar
            ((right_dual.group0().zxy() * anti_wedge.group0().yzx()) - (right_dual.group0().yzx() * anti_wedge.group0().zxy())).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * -1.0) * Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 0.0]).xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ) * Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       12        0
    //    simd4        2        7        0
    // Totals...
    // yes simd       12       19        0
    //  no simd       18       40        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * other.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e1234], other[e1234], other[e1234], 1.0])
                * self.group1().with_w(
                    -(self[e41] * other[e23])
                        - (self[e42] * other[e31])
                        - (self[e43] * other[e12])
                        - (self[e23] * other[e41])
                        - (self[e31] * other[e42])
                        - (self[e12] * other[e43]),
                ),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group0())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * right_dual[e23])
                        - (anti_wedge[e42] * right_dual[e31])
                        - (anti_wedge[e43] * right_dual[e12])
                        - (anti_wedge[e23] * right_dual[e41])
                        - (anti_wedge[e31] * right_dual[e42])
                        - (anti_wedge[e12] * right_dual[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd3        6       12        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       48       66        0
    //  no simd       78      108        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8       15        0
    //  no simd        8       19        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * -1.0)
                * Simd32x4::from([
                    (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                    (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                    (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                    -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
                ])
                .xyz()
                * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Div<reject_via_origin_from> for Motor {
    type Output = reject_via_origin_from_partial<Motor>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (f32::powi(other[scalar], 2) * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectViaOriginFrom<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       28       41        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e431] * self[e23]) + (other[e321] * self[e43]),
                -(other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0())
                - (other.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (anti_wedge.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e2] * right_dual[e431]) + (anti_wedge[e3] * right_dual[e412]) + (anti_wedge[e4] * right_dual[e321])
                        - (anti_wedge[e431] * right_dual[e2])
                        - (anti_wedge[e412] * right_dual[e3])
                        - (anti_wedge[e321] * right_dual[e4]),
                )
                - (right_dual.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().yzx() * right_dual.group0().zxy()) - (anti_wedge.group0().zxy() * right_dual.group0().yzx())).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       18        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0) * anti_wedge.group0().xyz().with_w(anti_wedge[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       12        0
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       10       35        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * other.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * other.group1().with_w(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 1.0])
                * right_dual.group0().with_w(
                    -(right_dual[e41] * anti_wedge[e23])
                        - (right_dual[e42] * anti_wedge[e31])
                        - (right_dual[e43] * anti_wedge[e12])
                        - (right_dual[e23] * anti_wedge[e41])
                        - (right_dual[e31] * anti_wedge[e42])
                        - (right_dual[e12] * anti_wedge[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       15       20        0
    //  no simd       29       39        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * other.group0().xyz())).with_w(other[e1234] * self[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * self[e23])
                        - (other[e42] * self[e31])
                        - (other[e43] * self[e12])
                        - (other[e23] * self[e41])
                        - (other[e31] * self[e42])
                        - (other[e12] * self[e43]),
                ),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group0())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * right_dual[e23])
                        - (anti_wedge[e42] * right_dual[e31])
                        - (anti_wedge[e43] * right_dual[e12])
                        - (anti_wedge[e23] * right_dual[e41])
                        - (anti_wedge[e31] * right_dual[e42])
                        - (anti_wedge[e12] * right_dual[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       51        0
    //    simd3        8       14        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       53       73        0
    //  no simd       90      125        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
                (self[e1234] * other[e1]) + (self[e31] * other[e412]),
                (self[e1234] * other[e2]) + (self[e12] * other[e423]),
                (self[e1234] * other[e3]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) + (self.group0() * other.group4().www().with_w(other[e4]))
                - (other.group4().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group2()) + (Simd32x3::from(other[e1234]) * self.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group3()) + (Simd32x3::from(other[e1234]) * self.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       25        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0) * anti_wedge.group0().xyz().with_w(anti_wedge[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl RejectViaOriginFrom<Point> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return AntiScalar::from_groups(
            // e1234
            (right_dual[e423] * anti_wedge[e1]) + (right_dual[e431] * anti_wedge[e2]) + (right_dual[e412] * anti_wedge[e3]) + (right_dual[e321] * anti_wedge[e4]),
        );
    }
}
impl RejectViaOriginFrom<Scalar> for Motor {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<reject_via_origin_from> for MultiVector {
    type Output = reject_via_origin_from_partial<MultiVector>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ (f32::powi(other[scalar], 2) * self[e1234]) + (other[scalar] * other[e1234] * self[scalar]));
    }
}
impl RejectViaOriginFrom<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd3        4        8        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       47        0
    //  no simd       50       81        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
                (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e431] * self[e23]) + (other[e321] * self[e43]),
                -(other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0())
                - (other.group1().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (other.group1().yzx() * self.group4().zxy()) - (other.group1().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * other.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[e423] * anti_wedge[e1]) + (right_dual[e431] * anti_wedge[e2]) + (right_dual[e412] * anti_wedge[e3]) + (right_dual[e321] * anti_wedge[e4])
                    - (right_dual[e1] * anti_wedge[e423])
                    - (right_dual[e2] * anti_wedge[e431])
                    - (right_dual[e3] * anti_wedge[e412])
                    - (right_dual[e4] * anti_wedge[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (right_dual.group0().zxy() * anti_wedge.group1().yzx()) - (right_dual.group0().yzx() * anti_wedge.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e3] * anti_wedge[e42]) + (right_dual[e4] * anti_wedge[e23]),
                (right_dual[e1] * anti_wedge[e43]) + (right_dual[e4] * anti_wedge[e31]),
                (right_dual[e2] * anti_wedge[e41]) + (right_dual[e4] * anti_wedge[e12]),
                -(right_dual[e2] * anti_wedge[e31]) - (right_dual[e3] * anti_wedge[e12]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1())
                - (right_dual.group0().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       14        0
    //  no simd        0       34        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group2().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * self[e1234]),
        );
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, anti_wedge[e321] * right_dual[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0]) * anti_wedge.group3().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       20        0
    //    simd3        0        7        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       26       45        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(right_dual[e41] * anti_wedge[e23])
                    - (right_dual[e42] * anti_wedge[e31])
                    - (right_dual[e43] * anti_wedge[e12])
                    - (right_dual[e23] * anti_wedge[e41])
                    - (right_dual[e31] * anti_wedge[e42])
                    - (right_dual[e12] * anti_wedge[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge[scalar]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * anti_wedge.group1().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group1().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        3       10        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       47       71        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
                (other[e1234] * self[e1]) + (other[e31] * self[e412]),
                (other[e1234] * self[e2]) + (other[e12] * self[e423]),
                (other[e1234] * self[e3]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) + (other.group0() * self.group4().www().with_w(self[e4]))
                - (self.group4().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * other.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(self[e1234]) * other.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[e1234] * anti_wedge[scalar]) + (right_dual[scalar] * anti_wedge[e1234])
                    - (right_dual[e41] * anti_wedge[e23])
                    - (right_dual[e42] * anti_wedge[e31])
                    - (right_dual[e43] * anti_wedge[e12])
                    - (right_dual[e23] * anti_wedge[e41])
                    - (right_dual[e31] * anti_wedge[e42])
                    - (right_dual[e12] * anti_wedge[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2()) + (Simd32x3::from(anti_wedge[scalar]) * right_dual.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[scalar]) * anti_wedge.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (right_dual.group0().yzx() * anti_wedge.group1().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group1().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       68        0
    //    simd3       12       19        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       76       97        0
    //  no simd      130      165        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
                (other[e41] * self[e321]) + (other[e31] * self[e412]) + (other[e412] * self[e31]) + (other[e321] * self[e41]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) + (other[e423] * self[e12]) + (other[e321] * self[e42]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e431] * self[e23]) + (other[e321] * self[e43]),
                -(other[e43] * self[e412]) - (other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1())
                - (self.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (self.group3().zxy() * other.group4().yzx()).with_w(other[e42] * self[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * other.group2()) + (other.group4().yzx() * self.group4().zxy())
                - (other.group4().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3()) + (Simd32x3::from(other[e321]) * self.group4().xyz()) + (Simd32x3::from(self[e1234]) * other.group3())
                - (Simd32x3::from(self[e321]) * other.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4()) + (Simd32x4::from(self[e1234]) * other.group4()),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd2        0        1        0
    //    simd3        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       17       51        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]),
                -(self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) - (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (self.group4().zxy() * other.group0().yzx()) - (self.group4().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, anti_wedge[e321] * right_dual[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0]) * anti_wedge.group3().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       24        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[e1] * right_dual[e423]) + (anti_wedge[e2] * right_dual[e431]) + (anti_wedge[e3] * right_dual[e412]) + (anti_wedge[e4] * right_dual[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            anti_wedge.group0().xx().with_zw(anti_wedge[scalar], 0.0)
                * Simd32x3::from(1.0).with_w(0.0)
                * right_dual.group0().xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Scalar> for MultiVector {
    type Output = AntiScalar;
    fn reject_via_origin_from(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * f32::powi(other[scalar], 2));
    }
}
impl std::ops::Div<reject_via_origin_from> for Origin {
    type Output = reject_via_origin_from_partial<Origin>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group1(),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ f32::powi(other[e321], 2) * self[e4] * -1.0);
    }
}
impl RejectViaOriginFrom<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual[scalar] * anti_wedge[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([anti_wedge[e4], anti_wedge[e4], anti_wedge[e4], 0.0]) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd2        0        1        0
    //    simd3        6       17        0
    //    simd4        8        2        0
    // Totals...
    // yes simd       29       40        0
    //  no simd       65       81        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * right_dual[scalar],
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group3()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[e4]) * anti_wedge.group3()).with_w(0.0)
                + (anti_wedge.group2().yzx() * right_dual.group1().zxy()).with_w(0.0)
                + (right_dual.group2().yzx() * anti_wedge.group1().zxy()).with_w(0.0)
                - (anti_wedge.group2().zxy() * right_dual.group1().yzx()).with_w(0.0)
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * f32::powi(other[e321], 2) * -1.0);
    }
}
impl std::ops::Div<reject_via_origin_from> for Plane {
    type Output = reject_via_origin_from_partial<Plane>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       21       37        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().yzx() * self.group0().zxy()) - (other.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e321] * self[e423],
                other[e321] * self[e431],
                other[e321] * self[e412],
                -(other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) - (self.group0().wwwx() * other.group1().xyz().with_w(other[e1])),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e4] * anti_wedge[e23]) + (right_dual[e423] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e31]) + (right_dual[e431] * anti_wedge[scalar]),
                (right_dual[e4] * anti_wedge[e12]) + (right_dual[e412] * anti_wedge[scalar]),
                -(right_dual[e2] * anti_wedge[e31]) - (right_dual[e3] * anti_wedge[e12]),
            ]) + (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(right_dual[e321] * anti_wedge[scalar])
                - (right_dual.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0])
                * Simd32x3::from([other[e321] * self[e423], other[e321] * self[e431], other[e321] * self[e412]]).with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       16       24        0
    fn reject_via_origin_from(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
        );
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * anti_wedge.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Motor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       20       32        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group1().xyz()).with_w(0.0)
                + (anti_wedge.group0().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (anti_wedge.group0().yzx() * right_dual.group0().zxy()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       46        0
    //    simd3        8       14        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       48       67        0
    //  no simd       82      116        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]),
                -(other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (other.group4().yzx() * self.group0().zxy()) - (other.group4().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * right_dual[e42]) + (anti_wedge[e4] * right_dual[e23]) + (anti_wedge[e42] * right_dual[e3]) + (anti_wedge[e23] * right_dual[e4]),
                (anti_wedge[e1] * right_dual[e43]) + (anti_wedge[e4] * right_dual[e31]) + (anti_wedge[e43] * right_dual[e1]) + (anti_wedge[e31] * right_dual[e4]),
                (anti_wedge[e2] * right_dual[e41]) + (anti_wedge[e4] * right_dual[e12]) + (anti_wedge[e41] * right_dual[e2]) + (anti_wedge[e12] * right_dual[e4]),
                -(anti_wedge[e1] * right_dual[e23]) - (anti_wedge[e2] * right_dual[e31]) - (anti_wedge[e3] * right_dual[e12]) - (anti_wedge[e12] * right_dual[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * right_dual.group4())
                + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group4())
                - (right_dual.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * right_dual[e2]),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       15        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([right_dual[e4], right_dual[e4], right_dual[e4], 0.0])
                * Simd32x3::from([
                    (other[e321] * self[e423]) - (other[e423] * self[e321]),
                    (other[e321] * self[e431]) - (other[e431] * self[e321]),
                    (other[e321] * self[e412]) - (other[e412] * self[e321]),
                ])
                .with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn reject_via_origin_from(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(
            // scalar
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 0.0])
                * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]).xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl std::ops::Div<reject_via_origin_from> for Point {
    type Output = reject_via_origin_from_partial<Point>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       17        0
    fn reject_via_origin_from(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(
            // scalar
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
        );
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 0.0]) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl RejectViaOriginFrom<Horizon> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_via_origin_from(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ f32::powi(other[e321], 2) * self[e4] * -1.0);
    }
}
impl RejectViaOriginFrom<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        8       21        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz()).with_w(0.0) + (right_dual.group0().yzx() * anti_wedge.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * anti_wedge.group0().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       21        0
    //    simd3        6       17        0
    //    simd4        8        3        0
    // Totals...
    // yes simd       32       41        0
    //  no simd       68       84        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = MultiVector::from_groups(
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
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[scalar] * right_dual[e1234])
                    + (anti_wedge[e1234] * right_dual[scalar])
                    + (anti_wedge[e1] * right_dual[e423])
                    + (anti_wedge[e2] * right_dual[e431])
                    + (anti_wedge[e3] * right_dual[e412])
                    + (anti_wedge[e4] * right_dual[e321])
                    - (anti_wedge[e41] * right_dual[e23])
                    - (anti_wedge[e42] * right_dual[e31])
                    - (anti_wedge[e43] * right_dual[e12])
                    - (anti_wedge[e23] * right_dual[e41])
                    - (anti_wedge[e31] * right_dual[e42])
                    - (anti_wedge[e12] * right_dual[e43])
                    - (anti_wedge[e423] * right_dual[e1])
                    - (anti_wedge[e431] * right_dual[e2])
                    - (anti_wedge[e412] * right_dual[e3])
                    - (anti_wedge[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * right_dual.group1()) + (Simd32x4::from(right_dual[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group2())
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group1().xyz())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group2())
                - (Simd32x3::from(right_dual[e4]) * anti_wedge.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group3())
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group3())
                + (anti_wedge.group1().yzx() * right_dual.group1().zxy())
                - (anti_wedge.group1().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from(anti_wedge[scalar]) * right_dual.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(anti_wedge[e4]) * right_dual.group3()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * anti_wedge.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[e4]) * anti_wedge.group3()).with_w(0.0)
                + (anti_wedge.group2().yzx() * right_dual.group1().zxy()).with_w(0.0)
                + (right_dual.group2().yzx() * anti_wedge.group1().zxy()).with_w(0.0)
                - (anti_wedge.group2().zxy() * right_dual.group1().yzx()).with_w(0.0)
                - (right_dual.group2().zxy() * anti_wedge.group1().yzx()).with_w(0.0),
        );
    }
}
impl RejectViaOriginFrom<Plane> for Point {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn reject_via_origin_from(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(
            // e4
            -(f32::powi(other[e321], 2) * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3]),
        );
    }
}
impl std::ops::Div<reject_via_origin_from> for Scalar {
    type Output = reject_via_origin_from_partial<Scalar>;
    fn div(self, _rhs: reject_via_origin_from) -> Self::Output {
        reject_via_origin_from_partial(self)
    }
}
impl RejectViaOriginFrom<DualNum> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn reject_via_origin_from(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[scalar] * other[e1234] * self[scalar]);
    }
}
impl RejectViaOriginFrom<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn reject_via_origin_from(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group1(),
        );
    }
}
impl RejectViaOriginFrom<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       21        0
    fn reject_via_origin_from(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
        let right_dual = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[scalar]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e41, e42, e43
            other.group3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_wedge[scalar]) * right_dual.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group1(),
            // e41, e42, e43
            Simd32x3::from(anti_wedge[scalar]) * right_dual.group2(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge[scalar]) * right_dual.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * right_dual.group4(),
        );
    }
}
