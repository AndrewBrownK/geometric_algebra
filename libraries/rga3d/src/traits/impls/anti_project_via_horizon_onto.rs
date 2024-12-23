// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 86
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       9       0
//  Average:         9      16       0
//  Maximum:        76      98       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3      16       0
//  Average:        16      30       0
//  Maximum:       130     166       0
impl std::ops::Div<anti_project_via_horizon_onto> for AntiScalar {
    type Output = anti_project_via_horizon_onto_partial<AntiScalar>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[scalar], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Flector> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])
                        - (anti_wedge[e2] * other[e431])
                        - (anti_wedge[e3] * other[e412])
                        - (anti_wedge[e4] * other[e321]),
                )
                - (anti_wedge.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().zxy() * other.group0().yzx()) - (anti_wedge.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group1(),
        );
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge[e41] * other[e23])
                - (anti_wedge[e42] * other[e31])
                - (anti_wedge[e43] * other[e12])
                - (anti_wedge[e23] * other[e41])
                - (anti_wedge[e31] * other[e42])
                - (anti_wedge[e12] * other[e43]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       13       27        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * right_dual.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * other.group0())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * other[e23])
                        - (anti_wedge[e42] * other[e31])
                        - (anti_wedge[e43] * other[e12])
                        - (anti_wedge[e23] * other[e41])
                        - (anti_wedge[e31] * other[e42])
                        - (anti_wedge[e12] * other[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(anti_wedge[scalar] * other[scalar]),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for AntiScalar {
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
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * right_dual.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Point> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return AntiScalar::from_groups(
            // e1234
            (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[scalar], 2) * self[e1234]);
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for DualNum {
    type Output = anti_project_via_horizon_onto_partial<DualNum>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0());
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            anti_wedge[scalar] * other[scalar],
            (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]),
        ]));
    }
}
impl AntiProjectViaHorizonOnto<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        7        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       16       29        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])
                        - (anti_wedge[e2] * other[e431])
                        - (anti_wedge[e3] * other[e412])
                        - (anti_wedge[e4] * other[e321]),
                )
                - (anti_wedge.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().zxy() * other.group0().yzx()) - (anti_wedge.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group1(),
        );
        return AntiScalar::from_groups(
            // e1234
            -(anti_wedge[e41] * other[e23])
                - (anti_wedge[e42] * other[e31])
                - (anti_wedge[e43] * other[e12])
                - (anti_wedge[e23] * other[e41])
                - (anti_wedge[e31] * other[e42])
                - (anti_wedge[e12] * other[e43]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        1        2        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       17       35        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e23], right_dual[e31], right_dual[e12], 1.0])
                * self.group0().yy().with_zw(self[e1234], (self[scalar] * right_dual[e1234]) + (self[e1234] * right_dual[scalar])),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * other.group0())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * other[e23])
                        - (anti_wedge[e42] * other[e31])
                        - (anti_wedge[e43] * other[e12])
                        - (anti_wedge[e23] * other[e41])
                        - (anti_wedge[e31] * other[e42])
                        - (anti_wedge[e12] * other[e43]),
                ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(anti_wedge[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * anti_wedge.group1().xyz())).with_w(anti_wedge[scalar] * other[scalar]),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       38        0
    //    simd3        6       12        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       39       57        0
    //  no simd       66      102        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * right_dual[e1234]) + (self[e1234] * right_dual[scalar]), self[e1234] * right_dual[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * right_dual.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Point> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return AntiScalar::from_groups(
            // e1234
            (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * Simd32x2::from([right_dual[e1234] * self[scalar], right_dual[e1234] * self[e1234]]),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Flector {
    type Output = anti_project_via_horizon_onto_partial<Flector>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       28       41        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_dual.group1().yzx() * self.group1().zxy()) - (right_dual.group1().zxy() * self.group1().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            (right_dual.group1().wwwx() * self.group1().xyz().with_w(self[e1]))
                + Simd32x3::from(0.0).with_w(
                    (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                        - (right_dual[e2] * self[e431])
                        - (right_dual[e3] * self[e412])
                        - (right_dual[e4] * self[e321]),
                )
                - (self.group1().wwwx() * right_dual.group1().xyz().with_w(right_dual[e1])),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]),
                -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12]),
            ]) + (other.group0().zxy() * anti_wedge.group0().yzx()).with_w(other[e321] * anti_wedge[scalar])
                - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Flector {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[e321], 2) * self[e321]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       16       27        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e31]) + (self[e321] * right_dual[e41]),
                (self[e423] * right_dual[e12]) + (self[e321] * right_dual[e42]),
                (self[e431] * right_dual[e23]) + (self[e321] * right_dual[e43]),
                -(self[e431] * right_dual[e42]) - (self[e412] * right_dual[e43]),
            ]) - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]),
                (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]),
                (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        1        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       24       44        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e31]) + (self[e321] * right_dual[e41]),
                (self[e423] * right_dual[e12]) + (self[e321] * right_dual[e42]),
                (self[e431] * right_dual[e23]) + (self[e321] * right_dual[e43]),
                -(self[e431] * right_dual[e42]) - (self[e412] * right_dual[e43]),
            ]) + (Simd32x4::from(right_dual[e1234]) * self.group0())
                - (self.group1().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]),
                (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]),
                (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]),
                -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]),
            ]) + (anti_wedge.group0().zxy() * other.group0().yzx()).with_w(anti_wedge[e321] * other[scalar])
                - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       51        0
    //    simd3        8       14        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       53       73        0
    //  no simd       90      125        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321])
                    - (self[e423] * right_dual[e1])
                    - (self[e431] * right_dual[e2])
                    - (self[e412] * right_dual[e3])
                    - (self[e321] * right_dual[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e412] * right_dual[e31]) + (self[e321] * right_dual[e41]),
                (self[e423] * right_dual[e12]) + (self[e321] * right_dual[e42]),
                (self[e431] * right_dual[e23]) + (self[e321] * right_dual[e43]),
                -(self[e431] * right_dual[e42]) - (self[e412] * right_dual[e43]),
            ]) + (Simd32x4::from(right_dual[e1234]) * self.group0())
                - (self.group1().yzxx() * right_dual.group3().zxy().with_w(right_dual[e41])),
            // e41, e42, e43
            (self.group1().zxy() * right_dual.group4().yzx()) - (self.group1().yzx() * right_dual.group4().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e321]) * self.group1().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
    }
}
impl AntiProjectViaHorizonOnto<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       17       35        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group1().zxy() * right_dual.group0().yzx()) - (self.group1().yzx() * right_dual.group0().zxy())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self[e321] * right_dual[e423] * -1.0,
                self[e321] * right_dual[e431] * -1.0,
                self[e321] * right_dual[e412] * -1.0,
                (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321]),
            ]) + (right_dual.group0().wwwx() * self.group1().xyz().with_w(self[e1])),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Horizon {
    type Output = anti_project_via_horizon_onto_partial<Horizon>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[scalar], 2) * self[e321]);
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       29        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * right_dual.group1().xyz().with_w(right_dual[e4]) * Simd32x4::from(-1.0),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]),
                -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12]),
            ]) + (other.group0().zxy() * anti_wedge.group0().yzx()).with_w(other[e321] * anti_wedge[scalar])
                - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[e321], 2) * self[e321]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        8       23        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0])
                * Simd32x3::from([other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0]).with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]),
                (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]),
                (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd3        0        1        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       12       33        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * right_dual.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[e1234]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]),
                (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]),
                (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]),
                -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]),
            ]) + (anti_wedge.group0().zxy() * other.group0().yzx()).with_w(anti_wedge[e321] * other[scalar])
                - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Horizon {
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
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * right_dual[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * right_dual.group2().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * right_dual.group4().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[e321] * right_dual[e1234]),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e321] * other[e321]) * other.group0());
    }
}
impl AntiProjectViaHorizonOnto<Point> for Horizon {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x4::from([other[e1], other[e2], other[e3], 0.0]).xyz() * Simd32x3::from(-1.0),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[scalar], 2) * self[e321]);
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Line {
    type Output = anti_project_via_horizon_onto_partial<Line>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * anti_wedge.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       17       29        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e31]) + (right_dual[e321] * self[e41]),
                (right_dual[e423] * self[e12]) + (right_dual[e321] * self[e42]),
                (right_dual[e431] * self[e23]) + (right_dual[e321] * self[e43]),
                -(right_dual[e431] * self[e42]) - (right_dual[e412] * self[e43]),
            ]) - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[e4] * anti_wedge[e1],
                other[e4] * anti_wedge[e2],
                other[e4] * anti_wedge[e3],
                -(other[e431] * anti_wedge[e2]) - (other[e412] * anti_wedge[e3]) - (other[e321] * anti_wedge[e4]),
            ]) - (anti_wedge.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((other.group0().yzx() * anti_wedge.group0().zxy()) - (other.group0().zxy() * anti_wedge.group0().yzx())).with_w(0.0),
        );
    }
}
impl AntiProjectViaHorizonOnto<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       15        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Scalar::from_groups(
            // scalar
            -(right_dual[e41] * self[e23])
                - (right_dual[e42] * self[e31])
                - (right_dual[e43] * self[e12])
                - (right_dual[e23] * self[e41])
                - (right_dual[e31] * self[e42])
                - (right_dual[e12] * self[e43]),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(anti_wedge[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge[scalar]) * other.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       13        0
    //    simd3        1        2        0
    //    simd4        2        7        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       21       47        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * self.group0().with_w(0.0) * right_dual.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([right_dual[e1234], right_dual[e1234], right_dual[e1234], 1.0])
                * self.group1().with_w(
                    -(self[e41] * right_dual[e23])
                        - (self[e42] * right_dual[e31])
                        - (self[e43] * right_dual[e12])
                        - (self[e23] * right_dual[e41])
                        - (self[e31] * right_dual[e42])
                        - (self[e12] * right_dual[e43]),
                ),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * other.group0())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * other[e23])
                        - (anti_wedge[e42] * other[e31])
                        - (anti_wedge[e43] * other[e12])
                        - (anti_wedge[e23] * other[e41])
                        - (anti_wedge[e31] * other[e42])
                        - (anti_wedge[e12] * other[e43]),
                ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(anti_wedge[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * anti_wedge.group1().xyz())).with_w(anti_wedge[scalar] * other[scalar]),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd3        6       12        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       48       67        0
    //  no simd       78      109        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e41] * right_dual[e23])
                    - (self[e42] * right_dual[e31])
                    - (self[e43] * right_dual[e12])
                    - (self[e23] * right_dual[e41])
                    - (self[e31] * right_dual[e42])
                    - (self[e12] * right_dual[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e321]) + (self[e31] * right_dual[e412]),
                (self[e42] * right_dual[e321]) + (self[e12] * right_dual[e423]),
                (self[e43] * right_dual[e321]) + (self[e23] * right_dual[e431]),
                -(self[e42] * right_dual[e431]) - (self[e43] * right_dual[e412]),
            ]) - (right_dual.group4().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Point> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        2        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       14       24        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e321]) + (self[e31] * right_dual[e412]),
                (self[e42] * right_dual[e321]) + (self[e12] * right_dual[e423]),
                (self[e43] * right_dual[e321]) + (self[e23] * right_dual[e431]),
                -(self[e42] * right_dual[e431]) - (self[e43] * right_dual[e412]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
        );
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge.group0().xyz()) - (Simd32x3::from(anti_wedge[e4]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge.group0().zxy() * other.group0().yzx()) - (anti_wedge.group0().yzx() * other.group0().zxy()),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group1(),
        );
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * anti_wedge.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Motor {
    type Output = anti_project_via_horizon_onto_partial<Motor>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1       18        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([anti_wedge[e41], anti_wedge[e42], anti_wedge[e43], 1.0])
                * other.group0().xx().with_zw(other[scalar], (other[scalar] * anti_wedge[e1234]) + (other[e1234] * anti_wedge[scalar])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       28       41        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e31]) + (right_dual[e321] * self[e41]),
                (right_dual[e423] * self[e12]) + (right_dual[e321] * self[e42]),
                (right_dual[e431] * self[e23]) + (right_dual[e321] * self[e43]),
                -(right_dual[e431] * self[e42]) - (right_dual[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * right_dual.group0())
                - (right_dual.group1().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (other.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423]))
                + Simd32x3::from(0.0).with_w(
                    (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])
                        - (anti_wedge[e2] * other[e431])
                        - (anti_wedge[e3] * other[e412])
                        - (anti_wedge[e4] * other[e321]),
                )
                - (anti_wedge.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().zxy() * other.group0().yzx()) - (anti_wedge.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       12        0
    //    simd3        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       10       47        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group0().with_w(0.0) * self.group0().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 1.0])
                * right_dual.group1().with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 1.0])
                * other.group0().with_w(
                    -(other[e41] * anti_wedge[e23])
                        - (other[e42] * anti_wedge[e31])
                        - (other[e43] * anti_wedge[e12])
                        - (other[e23] * anti_wedge[e41])
                        - (other[e31] * anti_wedge[e42])
                        - (other[e12] * anti_wedge[e43]),
                ),
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * other.group1().with_w(0.0) * anti_wedge.group1().www().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        2        4        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       16       23        0
    //  no simd       32       46        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(right_dual[e1234]) * self.group0().xyz()) + (Simd32x3::from(self[e1234]) * right_dual.group0().xyz())).with_w(right_dual[e1234] * self[e1234]),
            // e23, e31, e12, scalar
            (Simd32x4::from(right_dual[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * right_dual.group1())
                + Simd32x3::from(0.0).with_w(
                    -(right_dual[e41] * self[e23])
                        - (right_dual[e42] * self[e31])
                        - (right_dual[e43] * self[e12])
                        - (right_dual[e23] * self[e41])
                        - (right_dual[e31] * self[e42])
                        - (right_dual[e12] * self[e43]),
                ),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(anti_wedge[scalar]) * other.group0())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group0())
                + Simd32x3::from(0.0).with_w(
                    -(anti_wedge[e41] * other[e23])
                        - (anti_wedge[e42] * other[e31])
                        - (anti_wedge[e43] * other[e12])
                        - (anti_wedge[e23] * other[e41])
                        - (anti_wedge[e31] * other[e42])
                        - (anti_wedge[e12] * other[e43]),
                ),
            // e23, e31, e12, scalar
            ((Simd32x3::from(anti_wedge[scalar]) * other.group1().xyz()) + (Simd32x3::from(other[scalar]) * anti_wedge.group1().xyz())).with_w(anti_wedge[scalar] * other[scalar]),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       52        0
    //    simd3        8       14        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       90      126        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1234] * right_dual[scalar]) + (self[scalar] * right_dual[e1234])
                    - (self[e41] * right_dual[e23])
                    - (self[e42] * right_dual[e31])
                    - (self[e43] * right_dual[e12])
                    - (self[e23] * right_dual[e41])
                    - (self[e31] * right_dual[e42])
                    - (self[e12] * right_dual[e43]),
                self[e1234] * right_dual[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e1234] * right_dual[e1]) + (self[e31] * right_dual[e412]),
                (self[e1234] * right_dual[e2]) + (self[e12] * right_dual[e423]),
                (self[e1234] * right_dual[e3]) + (self[e23] * right_dual[e431]),
                -(self[e42] * right_dual[e431]) - (self[e43] * right_dual[e412]),
            ]) + (self.group0() * right_dual.group4().www().with_w(right_dual[e4]))
                - (right_dual.group4().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * right_dual.group2()) + (Simd32x3::from(right_dual[e1234]) * self.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * right_dual.group3()) + (Simd32x3::from(right_dual[e1234]) * self.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ f32::powi(other[e321], 2) * self[e1234]);
    }
}
impl AntiProjectViaHorizonOnto<Point> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       17       35        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e321]) + (self[e31] * right_dual[e412]),
                (self[e42] * right_dual[e321]) + (self[e12] * right_dual[e423]),
                (self[e43] * right_dual[e321]) + (self[e23] * right_dual[e431]),
                -(self[e42] * right_dual[e431]) - (self[e43] * right_dual[e412]),
            ]) - (right_dual.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group0(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                anti_wedge[e4] * other[e1] * -1.0,
                anti_wedge[e4] * other[e2] * -1.0,
                anti_wedge[e4] * other[e3] * -1.0,
                (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]),
            ]) + (other.group0().wwwx() * anti_wedge.group0().xyz().with_w(anti_wedge[e423])),
            // e23, e31, e12, scalar
            ((anti_wedge.group0().zxy() * other.group0().yzx()) - (anti_wedge.group0().yzx() * other.group0().zxy())).with_w(0.0),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(right_dual[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(right_dual[e1234]) * self.group1(),
        );
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for MultiVector {
    type Output = anti_project_via_horizon_onto_partial<MultiVector>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1       12        0
    //  no simd        1       33        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(right_dual[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * anti_wedge[scalar], (other[scalar] * anti_wedge[e1234]) + (other[e1234] * anti_wedge[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * anti_wedge.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * anti_wedge.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * anti_wedge.group4(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd3        4        8        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       47        0
    //  no simd       50       81        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])
                    - (right_dual[e1] * self[e423])
                    - (right_dual[e2] * self[e431])
                    - (right_dual[e3] * self[e412])
                    - (right_dual[e4] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e412] * self[e31]) + (right_dual[e321] * self[e41]),
                (right_dual[e423] * self[e12]) + (right_dual[e321] * self[e42]),
                (right_dual[e431] * self[e23]) + (right_dual[e321] * self[e43]),
                -(right_dual[e431] * self[e42]) - (right_dual[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * right_dual.group0())
                - (right_dual.group1().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (right_dual.group1().yzx() * self.group4().zxy()) - (right_dual.group1().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group1(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (other[e1] * anti_wedge[e423]) + (other[e2] * anti_wedge[e431]) + (other[e3] * anti_wedge[e412]) + (other[e4] * anti_wedge[e321])
                    - (other[e423] * anti_wedge[e1])
                    - (other[e431] * anti_wedge[e2])
                    - (other[e412] * anti_wedge[e3])
                    - (other[e321] * anti_wedge[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz()) - (Simd32x3::from(anti_wedge[e4]) * other.group0().xyz()),
            // e23, e31, e12
            (other.group0().yzx() * anti_wedge.group1().zxy()) - (other.group0().zxy() * anti_wedge.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * anti_wedge[e42]) + (other[e4] * anti_wedge[e23]),
                (other[e1] * anti_wedge[e43]) + (other[e4] * anti_wedge[e31]),
                (other[e2] * anti_wedge[e41]) + (other[e4] * anti_wedge[e12]),
                -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group1())
                - (other.group0().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * right_dual[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e321] * anti_wedge[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[e321] * anti_wedge[scalar]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       28        0
    //    simd3        0        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       26       51        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e41] * self[e321]) + (right_dual[e31] * self[e412]),
                (right_dual[e42] * self[e321]) + (right_dual[e12] * self[e423]),
                (right_dual[e43] * self[e321]) + (right_dual[e23] * self[e431]),
                -(right_dual[e42] * self[e431]) - (right_dual[e43] * self[e412]),
            ]) - (self.group4().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * right_dual.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * right_dual.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(other[e41] * anti_wedge[e23])
                    - (other[e42] * anti_wedge[e31])
                    - (other[e43] * anti_wedge[e12])
                    - (other[e23] * anti_wedge[e41])
                    - (other[e31] * anti_wedge[e42])
                    - (other[e12] * anti_wedge[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(anti_wedge[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge[scalar]) * other.group1(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]),
                (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]),
                (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) - (anti_wedge.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        4        9        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       30       50        0
    //  no simd       50       86        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_dual[e1234] * self[scalar]) + (right_dual[scalar] * self[e1234])
                    - (right_dual[e41] * self[e23])
                    - (right_dual[e42] * self[e31])
                    - (right_dual[e43] * self[e12])
                    - (right_dual[e23] * self[e41])
                    - (right_dual[e31] * self[e42])
                    - (right_dual[e12] * self[e43]),
                right_dual[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e1234] * self[e1]) + (right_dual[e31] * self[e412]),
                (right_dual[e1234] * self[e2]) + (right_dual[e12] * self[e423]),
                (right_dual[e1234] * self[e3]) + (right_dual[e23] * self[e431]),
                -(right_dual[e42] * self[e431]) - (right_dual[e43] * self[e412]),
            ]) + (right_dual.group0() * self.group4().www().with_w(self[e4]))
                - (self.group4().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * right_dual.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group3()) + (Simd32x3::from(self[e1234]) * right_dual.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[scalar] * anti_wedge[scalar],
                (other[e1234] * anti_wedge[scalar]) + (other[scalar] * anti_wedge[e1234])
                    - (other[e41] * anti_wedge[e23])
                    - (other[e42] * anti_wedge[e31])
                    - (other[e43] * anti_wedge[e12])
                    - (other[e23] * anti_wedge[e41])
                    - (other[e31] * anti_wedge[e42])
                    - (other[e12] * anti_wedge[e43]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(anti_wedge[scalar]) * other.group0().xyz()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (Simd32x3::from(anti_wedge[scalar]) * other.group1().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e23] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e423]),
                (other[e31] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e431]),
                (other[e12] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e412]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) + (other.group0().yzx() * anti_wedge.group1().zxy()).with_w(other[scalar] * anti_wedge[e321])
                - (anti_wedge.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       69        0
    //    simd3       12       19        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       76       98        0
    //  no simd      130      166        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
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
                right_dual[e1234] * self[e1234],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e41] * self[e321]) + (right_dual[e31] * self[e412]) + (right_dual[e412] * self[e31]) + (right_dual[e321] * self[e41]),
                (right_dual[e42] * self[e321]) + (right_dual[e12] * self[e423]) + (right_dual[e423] * self[e12]) + (right_dual[e321] * self[e42]),
                (right_dual[e43] * self[e321]) + (right_dual[e23] * self[e431]) + (right_dual[e431] * self[e23]) + (right_dual[e321] * self[e43]),
                -(right_dual[e43] * self[e412]) - (right_dual[e423] * self[e41]) - (right_dual[e431] * self[e42]) - (right_dual[e412] * self[e43]),
            ]) + (Simd32x4::from(right_dual[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * right_dual.group1())
                - (self.group4().yzxx() * right_dual.group3().zxy().with_w(right_dual[e41]))
                - (self.group3().zxy() * right_dual.group4().yzx()).with_w(right_dual[e42] * self[e431]),
            // e41, e42, e43
            (Simd32x3::from(right_dual[e1234]) * self.group2()) + (Simd32x3::from(self[e1234]) * right_dual.group2()) + (right_dual.group4().yzx() * self.group4().zxy())
                - (right_dual.group4().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e1234]) * self.group3()) + (Simd32x3::from(right_dual[e321]) * self.group4().xyz()) + (Simd32x3::from(self[e1234]) * right_dual.group3())
                - (Simd32x3::from(self[e321]) * right_dual.group4().xyz()),
            // e423, e431, e412, e321
            (Simd32x4::from(right_dual[e1234]) * self.group4()) + (Simd32x4::from(self[e1234]) * right_dual.group4()),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       13        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let right_dual = Origin::from_groups(/* e4 */ other[e321] * -1.0);
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * right_dual[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * right_dual[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                -(anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       24        0
    //    simd3        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       34       64        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * right_dual[e423]) + (self[e2] * right_dual[e431]) + (self[e3] * right_dual[e412]) + (self[e4] * right_dual[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * right_dual[e321]) + (self[e31] * right_dual[e412]),
                (self[e42] * right_dual[e321]) + (self[e12] * right_dual[e423]),
                (self[e43] * right_dual[e321]) + (self[e23] * right_dual[e431]),
                -(self[e42] * right_dual[e431]) - (self[e43] * right_dual[e412]),
            ]) - (right_dual.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (self.group4().zxy() * right_dual.group0().yzx()) - (self.group4().yzx() * right_dual.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e321]) * self.group4().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group0().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * right_dual.group0(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz()) - (Simd32x3::from(anti_wedge[e4]) * other.group0().xyz()),
            // e23, e31, e12
            (anti_wedge.group1().zxy() * other.group0().yzx()) - (anti_wedge.group1().yzx() * other.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       32        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(right_dual[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(right_dual[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(right_dual[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group4(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * anti_wedge.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * anti_wedge.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * anti_wedge.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * anti_wedge.group4(),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Origin {
    type Output = anti_project_via_horizon_onto_partial<Origin>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ f32::powi(other[scalar], 2) * self[e4]);
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ self[e4] * 0.0);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * other.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Origin::from_groups(/* e4 */ (other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))[3] * self[e4]);
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * anti_wedge[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([anti_wedge[e4], anti_wedge[e4], anti_wedge[e4], 0.0]) * other.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       37        0
    //    simd2        0        1        0
    //    simd3        6       10        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       38       53        0
    //  no simd       65       89        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([right_dual[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_dual[e1234] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Point> for Origin {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e4] * 0.0) * other.group0());
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ f32::powi(other[scalar], 2) * self[e4]);
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Plane {
    type Output = anti_project_via_horizon_onto_partial<Plane>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar])
                * Simd32x4::from([
                    right_dual[e1234] * self[e423],
                    right_dual[e1234] * self[e431],
                    right_dual[e1234] * self[e412],
                    right_dual[e1234] * self[e321],
                ]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd3        1        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       21       37        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Motor::from_groups(
            // e41, e42, e43, e1234
            ((right_dual.group1().yzx() * self.group0().zxy()) - (right_dual.group1().zxy() * self.group0().yzx())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                right_dual[e321] * self[e423],
                right_dual[e321] * self[e431],
                right_dual[e321] * self[e412],
                -(right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]) - (right_dual[e4] * self[e321]),
            ]) - (self.group0().wwwx() * right_dual.group1().xyz().with_w(right_dual[e1])),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]),
                (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]),
                -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12]),
            ]) + (other.group0().zxy() * anti_wedge.group0().yzx()).with_w(other[e321] * anti_wedge[scalar])
                - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Horizon> for Plane {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ f32::powi(other[e321], 2) * self[e321]);
    }
}
impl AntiProjectViaHorizonOnto<Line> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       16       27        0
    fn anti_project_via_horizon_onto(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let right_dual = Line::from_groups(/* e41, e42, e43 */ other.group1() * Simd32x3::from(-1.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
        let anti_wedge = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e41] * self[e321]) + (right_dual[e31] * self[e412]),
                (right_dual[e42] * self[e321]) + (right_dual[e12] * self[e423]),
                (right_dual[e43] * self[e321]) + (right_dual[e23] * self[e431]),
                -(right_dual[e42] * self[e431]) - (right_dual[e43] * self[e412]),
            ]) - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]),
                (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]),
                (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        1        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       20       40        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let right_dual = Motor::from_groups(
            // e41, e42, e43, e1234
            other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        let anti_wedge = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e41] * self[e321]) + (right_dual[e31] * self[e412]),
                (right_dual[e42] * self[e321]) + (right_dual[e12] * self[e423]),
                (right_dual[e43] * self[e321]) + (right_dual[e23] * self[e431]),
                -(right_dual[e42] * self[e431]) - (right_dual[e43] * self[e412]),
            ]) - (self.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group0(),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]),
                (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]),
                (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]),
                -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]),
            ]) + (anti_wedge.group0().zxy() * other.group0().yzx()).with_w(anti_wedge[e321] * other[scalar])
                - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       47        0
    //    simd3        8       14        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       48       68        0
    //  no simd       82      117        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(right_dual[e1] * self[e423]) - (right_dual[e2] * self[e431]) - (right_dual[e3] * self[e412]) - (right_dual[e4] * self[e321]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (right_dual[e41] * self[e321]) + (right_dual[e31] * self[e412]),
                (right_dual[e42] * self[e321]) + (right_dual[e12] * self[e423]),
                (right_dual[e43] * self[e321]) + (right_dual[e23] * self[e431]),
                -(right_dual[e42] * self[e431]) - (right_dual[e43] * self[e412]),
            ]) - (self.group0().yzxx() * right_dual.group3().zxy().with_w(right_dual[e41])),
            // e41, e42, e43
            (right_dual.group4().yzx() * self.group0().zxy()) - (right_dual.group4().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group4().xyz()),
            // e423, e431, e412, e321
            Simd32x4::from(right_dual[e1234]) * self.group0(),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Plane> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_project_via_horizon_onto(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e321] * self[e321]) * other.group0());
    }
}
impl AntiProjectViaHorizonOnto<Point> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        2        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       14       24        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        let anti_wedge = Line::from_groups(
            // e41, e42, e43
            (right_dual.group0().yzx() * self.group0().zxy()) - (right_dual.group0().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(right_dual[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * right_dual.group0().xyz()),
        );
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3]),
            ]) - (other.group0().yzxx() * anti_wedge.group0().zxy().with_w(anti_wedge[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar])
                * Simd32x4::from([
                    right_dual[e1234] * self[e423],
                    right_dual[e1234] * self[e431],
                    right_dual[e1234] * self[e412],
                    right_dual[e1234] * self[e321],
                ]),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Point {
    type Output = anti_project_via_horizon_onto_partial<Point>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar])
                * Simd32x4::from([right_dual[e1234] * self[e1], right_dual[e1234] * self[e2], right_dual[e1234] * self[e3], right_dual[e1234] * self[e4]]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn anti_project_via_horizon_onto(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let right_dual = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = Scalar::from_groups(
            // scalar
            (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
        );
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * other.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0());
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * anti_wedge.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]),
                (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]),
                (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]),
                -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3]),
            ]) - (anti_wedge.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        6       10        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       41       55        0
    //  no simd       68       93        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
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
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
        let anti_wedge = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_dual[e1234]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                anti_wedge[scalar] * other[scalar],
                (anti_wedge[scalar] * other[e1234])
                    + (anti_wedge[e1234] * other[scalar])
                    + (anti_wedge[e423] * other[e1])
                    + (anti_wedge[e431] * other[e2])
                    + (anti_wedge[e412] * other[e3])
                    + (anti_wedge[e321] * other[e4])
                    - (anti_wedge[e1] * other[e423])
                    - (anti_wedge[e2] * other[e431])
                    - (anti_wedge[e3] * other[e412])
                    - (anti_wedge[e4] * other[e321])
                    - (anti_wedge[e41] * other[e23])
                    - (anti_wedge[e42] * other[e31])
                    - (anti_wedge[e43] * other[e12])
                    - (anti_wedge[e23] * other[e41])
                    - (anti_wedge[e31] * other[e42])
                    - (anti_wedge[e12] * other[e43]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(anti_wedge[scalar]) * other.group1()) + (Simd32x4::from(other[scalar]) * anti_wedge.group1()),
            // e41, e42, e43
            (Simd32x3::from(anti_wedge[scalar]) * other.group2()) + (Simd32x3::from(other[scalar]) * anti_wedge.group2()) + (Simd32x3::from(other[e4]) * anti_wedge.group1().xyz())
                - (Simd32x3::from(anti_wedge[e4]) * other.group1().xyz()),
            // e23, e31, e12
            (Simd32x3::from(anti_wedge[scalar]) * other.group3()) + (Simd32x3::from(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy() * other.group1().yzx())
                - (anti_wedge.group1().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]),
                (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]),
                (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]),
                -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3]),
            ]) + (Simd32x4::from(anti_wedge[scalar]) * other.group4())
                + (Simd32x4::from(other[scalar]) * anti_wedge.group4())
                - (other.group1().yzxx() * anti_wedge.group2().zxy().with_w(anti_wedge[e23]))
                - (other.group2().zxy() * anti_wedge.group1().yzx()).with_w(anti_wedge[e31] * other[e2]),
        );
    }
}
impl AntiProjectViaHorizonOnto<Point> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_project_via_horizon_onto(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let right_dual = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([other[e1], other[e2], other[e3], 0.0]));
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from((right_dual[e423] * self[e1]) + (right_dual[e431] * self[e2]) + (right_dual[e412] * self[e3]) + (right_dual[e321] * self[e4])) * other.group0(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiScalar::from_groups(/* e1234 */ other[scalar]);
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar])
                * Simd32x4::from([right_dual[e1234] * self[e1], right_dual[e1234] * self[e2], right_dual[e1234] * self[e3], right_dual[e1234] * self[e4]]),
        );
    }
}
impl std::ops::Div<anti_project_via_horizon_onto> for Scalar {
    type Output = anti_project_via_horizon_onto_partial<Scalar>;
    fn div(self, _rhs: anti_project_via_horizon_onto) -> Self::Output {
        anti_project_via_horizon_onto_partial(self)
    }
}
impl AntiProjectViaHorizonOnto<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_project_via_horizon_onto(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar] * self[scalar]) * other.group0());
    }
}
impl AntiProjectViaHorizonOnto<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn anti_project_via_horizon_onto(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ (other.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]))[3] * self[scalar]);
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(anti_wedge[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(anti_wedge[scalar]) * other.group1(),
        );
    }
}
impl AntiProjectViaHorizonOnto<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn anti_project_via_horizon_onto(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let anti_wedge = Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(anti_wedge[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(anti_wedge[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(anti_wedge[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(anti_wedge[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(anti_wedge[scalar]) * other.group4(),
        );
    }
}
impl AntiProjectViaHorizonOnto<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_project_via_horizon_onto(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(other[scalar], 2) * self[scalar]);
    }
}
