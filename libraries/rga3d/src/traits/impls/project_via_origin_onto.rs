// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 66
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      10       0
//  Average:         9      16       0
//  Maximum:        76      97       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      19       0
//  Average:        16      29       0
//  Maximum:       130     165       0
impl std::ops::Div<ProjectViaOriginOntoInfix> for DualNum {
    type Output = ProjectViaOriginOntoInfixPartial<DualNum>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       37        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().zxy() * wedge.group1().yzx()) - (other.group1().yzx() * wedge.group1().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge.group1().wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])
                        - (other[e431] * wedge[e2])
                        - (other[e412] * wedge[e3])
                        - (other[e321] * wedge[e4]),
                )
                - (other.group1().wwwx() * wedge.group1().xyz().with_w(wedge[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Line> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       17       31        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            self.group0().xx().with_zw(self[scalar], (self[scalar] * right_dual[e1234]) + (self[e1234] * right_dual[scalar])) * right_dual.group0().xyz().with_w(1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e1234]) * wedge.group0().xyz()) + (Simd32x3::from(wedge[e1234]) * other.group0().xyz())).with_w(other[e1234] * wedge[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * wedge[e23])
                        - (other[e42] * wedge[e31])
                        - (other[e43] * wedge[e12])
                        - (other[e23] * wedge[e41])
                        - (other[e31] * wedge[e42])
                        - (other[e12] * wedge[e43]),
                ),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       37        0
    //    simd3        6       11        0
    //    simd4        5        9        0
    // Totals...
    // yes simd       39       57        0
    //  no simd       66      106        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[scalar] * right_dual[e1234]) + (self[e1234] * right_dual[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group4().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Point> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       16        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * other.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return Scalar::from_groups(
            // scalar
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]),
        );
    }
}
impl ProjectViaOriginOnto<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Flector {
    type Output = ProjectViaOriginOntoInfixPartial<Flector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       28       41        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                        - (right_dual[e2] * self[e431])
                        - (right_dual[e3] * self[e412])
                        - (right_dual[e4] * self[e321]),
                )
                - (right_dual.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
            // e23, e31, e12, scalar
            ((right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy())).with_w(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(wedge[e1234]) * other.group0())
                - (other.group1().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * wedge.group0().xyz()).with_w(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * wedge[e1234]),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       16       24        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * self.group0().yzx()).with_w(0.0),
        );
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) - (wedge.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       20       40        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz()).with_w(0.0)
                + (self.group0().zxy() * right_dual.group0().yzx()).with_w(0.0)
                - (self.group0().yzx() * right_dual.group0().zxy()).with_w(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) - (wedge.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x3::from(1.0).with_w(0.0) * other.group0().www().with_w(0.0) * wedge.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       43        0
    //    simd3        8       18        0
    //    simd4        8        5        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       90      117        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321])
                    - (self[e423] * right_dual[e1])
                    - (self[e431] * right_dual[e2])
                    - (self[e412] * right_dual[e3])
                    - (self[e321] * right_dual[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (self.group0().yzx() * right_dual.group1().zxy()) - (self.group0().zxy() * right_dual.group1().yzx()),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group3()).with_w(0.0)
                + (Simd32x3::from(right_dual[scalar]) * self.group1().xyz()).with_w(0.0)
                + (right_dual.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        8       25        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321] * -1.0) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge[e41] * other[e321]) + (wedge[e31] * other[e412]),
                (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]),
                (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]),
                -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412]),
            ]) - (other.group0().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group0(),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Flector {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ other.group0().xyz().with_w(0.0));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321])) * other.group0(),
        );
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Horizon {
    type Output = ProjectViaOriginOntoInfixPartial<Horizon>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e1234 */ Simd32x3::from(0.0).with_w(other[e321] * -1.0)[3] * self[e321] * -1.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[e321], 2) * self[e321]);
    }
}
impl ProjectViaOriginOnto<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       19        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge = AntiScalar::from_groups(/* e1234 */ Simd32x3::from(0.0).with_w(other[e321] * -1.0)[3] * self[e321] * -1.0);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(wedge[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(wedge[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(wedge[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group4(),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Line {
    type Output = ProjectViaOriginOntoInfixPartial<Line>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       17       32        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e3] * self[e42]) + (right_dual[e4] * self[e23]),
                (right_dual[e1] * self[e43]) + (right_dual[e4] * self[e31]),
                (right_dual[e2] * self[e41]) + (right_dual[e4] * self[e12]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().zxy() * wedge.group0().yzx()) - (other.group1().yzx() * wedge.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e321] * wedge[e423] * -1.0,
                other[e321] * wedge[e431] * -1.0,
                other[e321] * wedge[e412] * -1.0,
                (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]),
            ]) + (wedge.group0().wwwx() * other.group1().xyz().with_w(other[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x4::from([self[e23] * right_dual[e4], self[e31] * right_dual[e4], self[e12] * right_dual[e4], 0.0]).xyz() * Simd32x3::from(-1.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let wedge = AntiScalar::from_groups(
            // e1234
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(wedge[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       18        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = AntiScalar::from_groups(
            // e1234
            -(self[e41] * right_dual[e23])
                - (self[e42] * right_dual[e31])
                - (self[e43] * right_dual[e12])
                - (self[e23] * right_dual[e41])
                - (self[e31] * right_dual[e42])
                - (self[e12] * right_dual[e43]),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(wedge[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        6       10        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       48       65        0
    //  no simd       78      103        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(self[e41] * right_dual[e23])
                    - (self[e42] * right_dual[e31])
                    - (self[e43] * right_dual[e12])
                    - (self[e23] * right_dual[e41])
                    - (self[e31] * right_dual[e42])
                    - (self[e12] * right_dual[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * right_dual[e3]) + (self[e23] * right_dual[e4]),
                (self[e43] * right_dual[e1]) + (self[e31] * right_dual[e4]),
                (self[e41] * right_dual[e2]) + (self[e12] * right_dual[e4]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) - (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        6       16        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(other[e321] * -1.0) * self.group1()).with_w(0.0));
        return Line::from_groups(
            // e41, e42, e43
            (other.group0().zxy() * wedge.group0().yzx()) - (other.group0().yzx() * wedge.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(wedge[e321]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * wedge.group0().xyz()),
        );
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Motor {
    type Output = ProjectViaOriginOntoInfixPartial<Motor>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for Motor {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       28       41        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e4] * self[e23]) + (right_dual[e423] * self[scalar]),
                (right_dual[e4] * self[e31]) + (right_dual[e431] * self[scalar]),
                (right_dual[e4] * self[e12]) + (right_dual[e412] * self[scalar]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (right_dual.group0().zxy() * self.group0().yzx()).with_w(right_dual[e321] * self[scalar])
                - (right_dual.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().zxy() * wedge.group1().yzx()) - (other.group1().yzx() * wedge.group1().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge.group1().wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])
                        - (other[e431] * wedge[e2])
                        - (other[e412] * wedge[e3])
                        - (other[e321] * wedge[e4]),
                )
                - (other.group1().wwwx() * wedge.group1().xyz().with_w(wedge[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()).with_w(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * wedge.group1().xyz().with_w(wedge[e4]) * Simd32x4::from(-1.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       12        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       10       33        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x3::from(self[scalar]) * right_dual.group0()).with_w(
                -(right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * other.group0().with_w(0.0) * wedge.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            (Simd32x3::from(wedge[e1234]) * other.group1()).with_w(
                -(other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43]),
            ),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       15       20        0
    //  no simd       29       39        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(right_dual[scalar]) * self.group0())
                + (Simd32x4::from(self[scalar]) * right_dual.group0())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e1234]) * wedge.group0().xyz()) + (Simd32x3::from(wedge[e1234]) * other.group0().xyz())).with_w(other[e1234] * wedge[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * wedge[e23])
                        - (other[e42] * wedge[e31])
                        - (other[e43] * wedge[e12])
                        - (other[e23] * wedge[e41])
                        - (other[e31] * wedge[e42])
                        - (other[e12] * wedge[e43]),
                ),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       52        0
    //    simd3        7       13        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       52       72        0
    //  no simd       87      119        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[e1234] * right_dual[scalar]) + (self[scalar] * right_dual[e1234])
                    - (self[e41] * right_dual[e23])
                    - (self[e42] * right_dual[e31])
                    - (self[e43] * right_dual[e12])
                    - (self[e23] * right_dual[e41])
                    - (self[e31] * right_dual[e42])
                    - (self[e12] * right_dual[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * right_dual.group2()) + (Simd32x3::from(right_dual[scalar]) * self.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e23] * right_dual[e4]) + (self[scalar] * right_dual[e423]),
                (self[e31] * right_dual[e4]) + (self[scalar] * right_dual[e431]),
                (self[e12] * right_dual[e4]) + (self[scalar] * right_dual[e412]),
                -(self[e31] * right_dual[e2]) - (self[e12] * right_dual[e3]),
            ]) + (self.group0().yzx() * right_dual.group1().zxy()).with_w(self[scalar] * right_dual[e321])
                - (right_dual.group1().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        1        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        9       21        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e4]),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[e4]) * self.group1().xyz()).with_w(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((wedge.group1().yzx() * other.group0().zxy()) - (wedge.group1().zxy() * other.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                wedge[e321] * other[e423],
                wedge[e321] * other[e431],
                wedge[e321] * other[e412],
                -(wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321]),
            ]) - (other.group0().wwwx() * wedge.group1().xyz().with_w(wedge[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       16        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x3::from(1.0).with_w(0.0) * self.group1().www().with_w(0.0) * other.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return Scalar::from_groups(
            // scalar
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]),
        );
    }
}
impl ProjectViaOriginOnto<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for MultiVector {
    type Output = ProjectViaOriginOntoInfixPartial<MultiVector>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for MultiVector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd3        4        8        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       47        0
    //  no simd       50       81        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                    - (right_dual[e1] * self[e423])
                    - (right_dual[e2] * self[e431])
                    - (right_dual[e3] * self[e412])
                    - (right_dual[e4] * self[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group0().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (right_dual.group0().zxy() * self.group1().yzx()) - (right_dual.group0().yzx() * self.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e3] * self[e42]) + (right_dual[e4] * self[e23]),
                (right_dual[e1] * self[e43]) + (right_dual[e4] * self[e31]),
                (right_dual[e2] * self[e41]) + (right_dual[e4] * self[e12]),
                -(right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * right_dual.group1())
                - (right_dual.group0().yzxx() * self.group2().zxy().with_w(self[e23])),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(wedge[e1234]) * other.group0())
                - (other.group1().yzxx() * wedge.group3().zxy().with_w(wedge[e41])),
            // e41, e42, e43
            (other.group1().zxy() * wedge.group4().yzx()) - (other.group1().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(wedge[e321]) * other.group1().xyz()) - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       27        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e321] * right_dual[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[e4]) * self.group3()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * wedge[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * wedge.group2()).with_w(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * wedge.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * wedge[e1234]),
        );
    }
}
impl ProjectViaOriginOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       20        0
    //    simd3        0        7        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       26       45        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * self.group1().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * self.group1().yzx()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) - (wedge.group4().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            Simd32x3::from(wedge[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(wedge[e1234]) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       25        0
    //    simd3        3       10        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       26       41        0
    //  no simd       47       79        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[e1234] * self[scalar]) + (right_dual[scalar] * self[e1234])
                    - (right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * right_dual.group0().xyz()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[scalar]) * self.group4().xyz()).with_w(0.0)
                + (Simd32x3::from(self[e4]) * right_dual.group1().xyz()).with_w(0.0)
                + (right_dual.group0().yzx() * self.group1().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * self.group1().yzx()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * wedge[scalar]) + (other[scalar] * wedge[e1234])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1234] * wedge[e1]) + (other[e31] * wedge[e412]),
                (other[e1234] * wedge[e2]) + (other[e12] * wedge[e423]),
                (other[e1234] * wedge[e3]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) + (other.group0() * wedge.group4().www().with_w(wedge[e4]))
                - (wedge.group4().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x3::from(1.0).with_w(0.0) * other.group0().www().with_w(0.0) * wedge.group4().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       68        0
    //    simd3       12       19        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       76       97        0
    //  no simd      130      165        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[scalar] * self[e1234])
                    + (right_dual[e1234] * self[scalar])
                    + (right_dual[e423] * self[e1])
                    + (right_dual[e431] * self[e2])
                    + (right_dual[e412] * self[e3])
                    + (right_dual[e321] * self[e4])
                    - (right_dual[e1] * self[e423])
                    - (right_dual[e2] * self[e431])
                    - (right_dual[e3] * self[e412])
                    - (right_dual[e4] * self[e321])
                    - (right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(right_dual[scalar]) * self.group1()) + (Simd32x4::from(self[scalar]) * right_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(right_dual[scalar]) * self.group2()) + (Simd32x3::from(self[scalar]) * right_dual.group2()) + (Simd32x3::from(self[e4]) * right_dual.group1().xyz())
                - (Simd32x3::from(right_dual[e4]) * self.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[scalar]) * self.group3()) + (Simd32x3::from(self[scalar]) * right_dual.group3()) + (right_dual.group1().zxy() * self.group1().yzx())
                - (right_dual.group1().yzx() * self.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (right_dual[e3] * self[e42]) + (right_dual[e4] * self[e23]) + (right_dual[e42] * self[e3]) + (right_dual[e23] * self[e4]),
                (right_dual[e1] * self[e43]) + (right_dual[e4] * self[e31]) + (right_dual[e43] * self[e1]) + (right_dual[e31] * self[e4]),
                (right_dual[e2] * self[e41]) + (right_dual[e4] * self[e12]) + (right_dual[e41] * self[e2]) + (right_dual[e12] * self[e4]),
                -(right_dual[e1] * self[e23]) - (right_dual[e2] * self[e31]) - (right_dual[e3] * self[e12]) - (right_dual[e12] * self[e3]),
            ]) + (Simd32x4::from(right_dual[scalar]) * self.group4())
                + (Simd32x4::from(self[scalar]) * right_dual.group4())
                - (self.group1().yzxx() * right_dual.group2().zxy().with_w(right_dual[e23]))
                - (self.group2().zxy() * right_dual.group1().yzx()).with_w(right_dual[e31] * self[e2]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd2        0        1        0
    //    simd3        2        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       17       46        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e321] * right_dual[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(right_dual[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(right_dual[e4]) * self.group3()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(wedge[e1] * other[e423]) - (wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge[e41] * other[e321]) + (wedge[e31] * other[e412]),
                (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]),
                (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]),
                -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412]),
            ]) - (other.group0().yzxx() * wedge.group3().zxy().with_w(wedge[e41])),
            // e41, e42, e43
            (wedge.group4().yzx() * other.group0().zxy()) - (wedge.group4().zxy() * other.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(wedge[e321]) * other.group0().xyz()) - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group0(),
        );
    }
}
impl ProjectViaOriginOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       24        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ other.group0().xyz().with_w(0.0));
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            self.group0().xx().with_zw(self[scalar], 0.0) * Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(wedge[e1234]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2) * self[scalar]);
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Origin {
    type Output = ProjectViaOriginOntoInfixPartial<Origin>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Origin {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let wedge = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) - (other.group1().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       21        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let wedge = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(wedge[e41] * other[e23])
                    - (wedge[e42] * other[e31])
                    - (wedge[e43] * other[e12])
                    - (wedge[e23] * other[e41])
                    - (wedge[e31] * other[e42])
                    - (wedge[e12] * other[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge[e41] * other[e321]) + (wedge[e31] * other[e412]),
                (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]),
                (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]),
                -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412]),
            ]) - (other.group4().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * wedge.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Plane {
    type Output = ProjectViaOriginOntoInfixPartial<Plane>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = AntiScalar::from_groups(
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]) - (right_dual[e4] * self[e321]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(wedge[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[e321], 2) * self[e321]);
    }
}
impl ProjectViaOriginOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       24        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = AntiScalar::from_groups(
            // e1234
            -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]) - (right_dual[e4] * self[e321]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(wedge[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(wedge[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(wedge[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(wedge[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group4(),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0());
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Point {
    type Output = ProjectViaOriginOntoInfixPartial<Point>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       21       40        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                right_dual[e4] * self[e1] * -1.0,
                right_dual[e4] * self[e2] * -1.0,
                right_dual[e4] * self[e3] * -1.0,
                (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
            ]) + (self.group0().wwwx() * right_dual.group0().xyz().with_w(right_dual[e423])),
            // e23, e31, e12, scalar
            ((right_dual.group0().zxy() * self.group0().yzx()) - (right_dual.group0().yzx() * self.group0().zxy())).with_w(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(wedge[e1234]) * other.group0())
                - (other.group1().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(wedge[e1234]) * other.group1(),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       10        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        return Point::from_groups(
            // e1, e2, e3, e4
            (Simd32x3::from(other[e321]) * Simd32x3::from([right_dual[e4] * self[e1] * -1.0, right_dual[e4] * self[e2] * -1.0, right_dual[e4] * self[e3] * -1.0])).with_w(0.0),
        );
    }
}
impl ProjectViaOriginOnto<Line> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        3        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       16       24        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group1()).with_w(0.0) + (right_dual.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * self.group0().yzx()).with_w(0.0),
        );
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) - (wedge.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        3        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       16       37        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let wedge = Plane::from_groups(
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()).with_w(0.0) + (right_dual.group0().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group0().zxy() * self.group0().yzx()).with_w(0.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]),
                -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412]),
            ]) - (wedge.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x3::from(1.0).with_w(0.0) * other.group0().www().with_w(0.0) * wedge.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        8       17        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       82      110        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * right_dual.group1().xyz()) - (Simd32x3::from(right_dual[e4]) * self.group0().xyz()),
            // e23, e31, e12
            (right_dual.group1().zxy() * self.group0().yzx()) - (right_dual.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e4]) * right_dual.group3()).with_w(0.0) + (right_dual.group2().yzx() * self.group0().zxy()).with_w(0.0)
                - (right_dual.group2().zxy() * self.group0().yzx()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       19        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e321] * -1.0) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (wedge[e41] * other[e321]) + (wedge[e31] * other[e412]),
                (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]),
                (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]),
                -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412]),
            ]) - (other.group0().yzxx() * wedge.group1().zxy().with_w(wedge[e41])),
        );
    }
}
impl ProjectViaOriginOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ other.group0().xyz().with_w(0.0));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])) * other.group0(),
        );
    }
}
impl std::ops::Div<ProjectViaOriginOntoInfix> for Scalar {
    type Output = ProjectViaOriginOntoInfixPartial<Scalar>;
    fn div(self, _rhs: ProjectViaOriginOntoInfix) -> Self::Output {
        ProjectViaOriginOntoInfixPartial(self)
    }
}
impl ProjectViaOriginOnto<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn project_via_origin_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl ProjectViaOriginOnto<Flector> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       28        0
    fn project_via_origin_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            other.group0().xyz().with_w(0.0),
        );
        let wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group0(),
            // e423, e431, e412, e321
            (Simd32x3::from(self[scalar]) * right_dual.group1().xyz()).with_w(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((other.group1().zxy() * wedge.group1().yzx()) - (other.group1().yzx() * wedge.group1().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            (wedge.group1().wwwx() * other.group1().xyz().with_w(other[e1]))
                + Simd32x3::from(0.0).with_w(
                    (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])
                        - (other[e431] * wedge[e2])
                        - (other[e412] * wedge[e3])
                        - (other[e321] * wedge[e4]),
                )
                - (other.group1().wwwx() * wedge.group1().xyz().with_w(wedge[e1])),
        );
    }
}
impl ProjectViaOriginOnto<Horizon> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Line> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn project_via_origin_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return Scalar::from_groups(
            // scalar
            -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]),
        );
    }
}
impl ProjectViaOriginOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       16       28        0
    fn project_via_origin_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * Simd32x4::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, other[scalar]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e1234]) * wedge.group0().xyz()) + (Simd32x3::from(wedge[e1234]) * other.group0().xyz())).with_w(other[e1234] * wedge[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                + Simd32x3::from(0.0).with_w(
                    -(other[e41] * wedge[e23])
                        - (other[e42] * wedge[e31])
                        - (other[e43] * wedge[e12])
                        - (other[e23] * wedge[e41])
                        - (other[e31] * wedge[e42])
                        - (other[e12] * wedge[e43]),
                ),
        );
    }
}
impl ProjectViaOriginOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       36        0
    //    simd2        0        1        0
    //    simd3        6       12        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       65       98        0
    fn project_via_origin_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
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
            other.group1().xyz().with_w(0.0),
        );
        let wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, right_dual[e1234] * self[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * right_dual.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * right_dual.group2(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (Simd32x3::from(self[scalar]) * right_dual.group4().xyz()).with_w(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[scalar] * wedge[e1234])
                    + (other[e1234] * wedge[scalar])
                    + (other[e1] * wedge[e423])
                    + (other[e2] * wedge[e431])
                    + (other[e3] * wedge[e412])
                    + (other[e4] * wedge[e321])
                    - (other[e41] * wedge[e23])
                    - (other[e42] * wedge[e31])
                    - (other[e43] * wedge[e12])
                    - (other[e23] * wedge[e41])
                    - (other[e31] * wedge[e42])
                    - (other[e12] * wedge[e43])
                    - (other[e423] * wedge[e1])
                    - (other[e431] * wedge[e2])
                    - (other[e412] * wedge[e3])
                    - (other[e321] * wedge[e4]),
                other[e1234] * wedge[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]),
                (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]),
                (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]),
                -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43]),
            ]) + (Simd32x4::from(other[e1234]) * wedge.group1())
                + (Simd32x4::from(wedge[e1234]) * other.group1())
                - (wedge.group4().yzxx() * other.group3().zxy().with_w(other[e41]))
                - (wedge.group3().zxy() * other.group4().yzx()).with_w(other[e42] * wedge[e431]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * wedge.group2()) + (Simd32x3::from(wedge[e1234]) * other.group2()) + (other.group4().zxy() * wedge.group4().yzx())
                - (other.group4().yzx() * wedge.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * wedge.group3()) + (Simd32x3::from(wedge[e1234]) * other.group3()) + (Simd32x3::from(wedge[e321]) * other.group4().xyz())
                - (Simd32x3::from(other[e321]) * wedge.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * wedge.group4()) + (Simd32x4::from(wedge[e1234]) * other.group4()),
        );
    }
}
impl ProjectViaOriginOnto<Plane> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[e321], 2) * self[scalar]);
    }
}
impl ProjectViaOriginOnto<Point> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        7        0
    fn project_via_origin_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let wedge = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x3::from(self[scalar]) * other.group0().xyz()).with_w(0.0));
        return Scalar::from_groups(
            // scalar
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]),
        );
    }
}
impl ProjectViaOriginOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn project_via_origin_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2) * self[scalar]);
    }
}
