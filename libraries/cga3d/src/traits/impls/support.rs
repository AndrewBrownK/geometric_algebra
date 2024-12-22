// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 24
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      12       0
//  Average:         6      14       0
//  Maximum:        25      50       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       4       0
//   Median:         9      27       0
//  Average:        12      30       0
//  Maximum:        49     102       0
impl std::ops::Div<support> for AntiCircleRotor {
    type Output = Sphere;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       31        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (Simd32x4::from(self_2[e5]) * right_anti_dual.group0().with_w(right_anti_dual[e321]))
                - (self_2.group0().yzxx() * right_anti_dual.group1().zxy().with_w(right_anti_dual[e235])),
            // e1234
            (right_anti_dual[e423] * self_2[e1]) + (right_anti_dual[e431] * self_2[e2]) + (right_anti_dual[e412] * self_2[e3]) + (right_anti_dual[e321] * self_2[e4]),
        );
    }
}
impl std::ops::Div<support> for AntiDipoleInversion {
    type Output = CircleRotor;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       24       50        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz()) + (right_anti_dual.group0().yzx() * self_2.group0().zxy())
                - (right_anti_dual.group0().zxy() * self_2.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_anti_dual[e41] * self_2[e5]) + (right_anti_dual[e15] * self_2[e4]),
                (right_anti_dual[e42] * self_2[e5]) + (right_anti_dual[e25] * self_2[e4]),
                (right_anti_dual[e43] * self_2[e5]) + (right_anti_dual[e35] * self_2[e4]),
                -(right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3]),
            ]) - (right_anti_dual.group1().wwwx() * self_2.group0().xyzx()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                right_anti_dual[e25] * self_2[e3] * -1.0,
                right_anti_dual[e35] * self_2[e1] * -1.0,
                right_anti_dual[e15] * self_2[e2] * -1.0,
                (right_anti_dual[e4315] * self_2[e2]) + (right_anti_dual[e4125] * self_2[e3]) + (right_anti_dual[e3215] * self_2[e4]),
            ]) + (Simd32x4::from(self_2[e5]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[e1234]))
                + (self_2.group0().yzxx() * right_anti_dual.group2().zxy().with_w(right_anti_dual[e4235])),
        );
    }
}
impl std::ops::Div<support> for AntiDualNum {
    type Output = FlatPoint;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DualNum::from_groups(/* e5, e12345 */ self.group0());
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(right_anti_dual[e5]) * self_2.group0());
    }
}
impl std::ops::Div<support> for AntiFlatPoint {
    type Output = Line;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz()) - (Simd32x3::from(right_anti_dual[e45]) * self_2.group0().xyz()),
            // e235, e315, e125
            (right_anti_dual.group0().zxy() * self_2.group0().yzx()) - (right_anti_dual.group0().yzx() * self_2.group0().zxy()),
        );
    }
}
impl std::ops::Div<support> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        9       27        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                right_anti_dual[e45] * self_2[e1] * -1.0,
                right_anti_dual[e45] * self_2[e2] * -1.0,
                right_anti_dual[e45] * self_2[e3] * -1.0,
                (right_anti_dual[e4315] * self_2[e2]) + (right_anti_dual[e4125] * self_2[e3]) + (right_anti_dual[e3215] * self_2[e4]),
            ]) + (self_2.group0().wwwx() * right_anti_dual.group0().xyz().with_w(right_anti_dual[e4235])),
            // e235, e315, e125, e5
            ((right_anti_dual.group0().zxy() * self_2.group0().yzx()) - (right_anti_dual.group0().yzx() * self_2.group0().zxy())).with_w(0.0),
        );
    }
}
impl std::ops::Div<support> for AntiLine {
    type Output = Plane;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (self_2.group0().yzxx() * right_anti_dual.group0().zxy().with_w(right_anti_dual[e235])),
        );
    }
}
impl std::ops::Div<support> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(right_anti_dual[e5]) * self_2.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (self_2.group0().yzxx() * right_anti_dual.group0().zxy().with_w(right_anti_dual[e235])),
        );
    }
}
impl std::ops::Div<support> for AntiPlane {
    type Output = AntiScalar;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return AntiScalar::from_groups(
            // e12345
            (right_anti_dual[e4235] * self_2[e1]) + (right_anti_dual[e4315] * self_2[e2]) + (right_anti_dual[e4125] * self_2[e3]) + (right_anti_dual[e3215] * self_2[e4]),
        );
    }
}
impl std::ops::Div<support> for AntiScalar {
    type Output = RoundPoint;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Scalar::from_groups(/* scalar */ self[e12345] * -1.0);
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0(),
            // e5
            self_2[e5] * right_anti_dual[scalar],
        );
    }
}
impl std::ops::Div<support> for Circle {
    type Output = Circle;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<support> for Circle {
    fn div_assign(&mut self, _rhs: support) {
        *self = self.support()
    }
}
impl Support for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       16        0
    //  no simd       20       34        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2(),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz()) + (right_anti_dual.group0().yzx() * self_2.group0().zxy())
                - (right_anti_dual.group0().zxy() * self_2.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_anti_dual[e41] * self_2[e5]) + (right_anti_dual[e15] * self_2[e4]),
                (right_anti_dual[e42] * self_2[e5]) + (right_anti_dual[e25] * self_2[e4]),
                (right_anti_dual[e43] * self_2[e5]) + (right_anti_dual[e35] * self_2[e4]),
                -(right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3]),
            ]) - (right_anti_dual.group1().wwwx() * self_2.group0().xyzx()),
            // e235, e315, e125
            (Simd32x3::from(self_2[e5]) * right_anti_dual.group1().xyz()) + (right_anti_dual.group2().zxy() * self_2.group0().yzx())
                - (right_anti_dual.group2().yzx() * self_2.group0().zxy()),
        );
    }
}
impl std::ops::Div<support> for CircleRotor {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        4        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       20       43        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz()) + (right_anti_dual.group0().yzx() * self_2.group0().zxy())
                - (right_anti_dual.group0().zxy() * self_2.group0().yzx()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_anti_dual[e41] * self_2[e5]) + (right_anti_dual[e15] * self_2[e4]),
                (right_anti_dual[e42] * self_2[e5]) + (right_anti_dual[e25] * self_2[e4]),
                (right_anti_dual[e43] * self_2[e5]) + (right_anti_dual[e35] * self_2[e4]),
                -(right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3]),
            ]) - (right_anti_dual.group1().wwwx() * self_2.group0().xyzx()),
            // e235, e315, e125, e4
            ((Simd32x3::from(self_2[e5]) * right_anti_dual.group1().xyz()) + (right_anti_dual.group2().zxy() * self_2.group0().yzx())
                - (right_anti_dual.group2().yzx() * self_2.group0().zxy()))
            .with_w(right_anti_dual[scalar] * self_2[e4]),
            // e1, e2, e3, e5
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0().xyz().with_w(self_2[e5]),
        );
    }
}
impl std::ops::Div<support> for Dipole {
    type Output = Sphere;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       15       30        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e321] * self_2[e5]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (Simd32x4::from([self_2[e5], self_2[e5], self_2[e5], self_2[e1]]) * right_anti_dual.group0().with_w(right_anti_dual[e235]))
                - (self_2.group0().yzxy() * right_anti_dual.group1().zxy().with_w(right_anti_dual[e315])),
            // e1234
            (right_anti_dual[e423] * self_2[e1]) + (right_anti_dual[e431] * self_2[e2]) + (right_anti_dual[e412] * self_2[e3]) + (right_anti_dual[e321] * self_2[e4]),
        );
    }
}
impl std::ops::Div<support> for DipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<support> for DipoleInversion {
    fn div_assign(&mut self, _rhs: support) {
        *self = self.support()
    }
}
impl Support for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        4        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       25       58        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group3().xyz()) - (Simd32x3::from(right_anti_dual[e4]) * self_2.group0().xyz()),
            // e23, e31, e12, e45
            (right_anti_dual.group3().zxyw() * self_2.group0().yzxw()) - (right_anti_dual.group3().yzx() * self_2.group0().zxy()).with_w(right_anti_dual[e4] * self_2[e5]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                right_anti_dual[e1] * self_2[e5] * -1.0,
                right_anti_dual[e2] * self_2[e5] * -1.0,
                right_anti_dual[e3] * self_2[e5] * -1.0,
                (right_anti_dual[e431] * self_2[e2]) + (right_anti_dual[e412] * self_2[e3]) + (right_anti_dual[e321] * self_2[e4]),
            ]) + (self_2.group0().xyzx() * right_anti_dual.group3().www().with_w(right_anti_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (Simd32x4::from(self_2[e5]) * right_anti_dual.group0().with_w(right_anti_dual[e321]))
                - (self_2.group0().yzxx() * right_anti_dual.group1().zxy().with_w(right_anti_dual[e235])),
        );
    }
}
impl std::ops::Div<support> for DualNum {
    type Output = VersorEven;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from(-1.0));
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(right_anti_dual[e3215] * self_2[e4]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x3::from(0.0).with_w(right_anti_dual[scalar] * self_2[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0(),
        );
    }
}
impl std::ops::Div<support> for FlatPoint {
    type Output = Sphere;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for FlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        3       13        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self_2[e4], self_2[e4], self_2[e4], 1.0])
                * right_anti_dual.group0().xyz().with_w(
                    -(right_anti_dual[e235] * self_2[e1]) - (right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]) - (right_anti_dual[e321] * self_2[e5]),
                ),
            // e1234
            right_anti_dual[e321] * self_2[e4],
        );
    }
}
impl std::ops::Div<support> for Flector {
    type Output = DipoleInversion;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        9       33        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual.group1().zxy() * self_2.group0().yzx()) - (right_anti_dual.group1().yzx() * self_2.group0().zxy())).with_w(right_anti_dual[e5] * self_2[e4]),
            // e15, e25, e35, e1234
            ((Simd32x3::from(right_anti_dual[e5]) * self_2.group0().xyz()) - (Simd32x3::from(self_2[e5]) * right_anti_dual.group1().xyz()))
                .with_w(right_anti_dual[e321] * self_2[e4]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self_2[e4], self_2[e4], self_2[e4], 1.0])
                * right_anti_dual.group0().xyz().with_w(
                    -(right_anti_dual[e235] * self_2[e1]) - (right_anti_dual[e315] * self_2[e2]) - (right_anti_dual[e125] * self_2[e3]) - (right_anti_dual[e321] * self_2[e5]),
                ),
        );
    }
}
impl std::ops::Div<support> for Line {
    type Output = Circle;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Line {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       19        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1());
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self_2[e4], self_2[e4], self_2[e4], 1.0])
                * right_anti_dual
                    .group1()
                    .with_w(-(right_anti_dual[e23] * self_2[e1]) - (right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3])),
            // e235, e315, e125
            (Simd32x3::from(self_2[e5]) * right_anti_dual.group0()) + (right_anti_dual.group1().zxy() * self_2.group0().yzx())
                - (right_anti_dual.group1().yzx() * self_2.group0().zxy()),
        );
    }
}
impl std::ops::Div<support> for Motor {
    type Output = VersorEven;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        2        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       33        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435, e321
            Simd32x4::from([self_2[e4], self_2[e4], self_2[e4], 1.0])
                * right_anti_dual
                    .group1()
                    .xyz()
                    .with_w(-(right_anti_dual[e23] * self_2[e1]) - (right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3])),
            // e235, e315, e125, e5
            ((Simd32x3::from(self_2[e5]) * right_anti_dual.group0().xyz()) + (right_anti_dual.group1().zxy() * self_2.group0().yzx())
                - (right_anti_dual.group1().yzx() * self_2.group0().zxy()))
            .with_w(right_anti_dual[scalar] * self_2[e5]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0(),
        );
    }
}
impl std::ops::Div<support> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<support> for MultiVector {
    fn div_assign(&mut self, _rhs: support) {
        *self = self.support()
    }
}
impl Support for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd2        0        1        0
    //    simd3        6       12        0
    //    simd4        4        9        0
    // Totals...
    // yes simd       25       50        0
    //  no simd       49      102        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = MultiVector::from_groups(
            // scalar, e12345
            self.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group9().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
            // e15, e25, e35, e45
            self.group8().with_w(self[e321] * -1.0),
            // e41, e42, e43
            self.group7(),
            // e23, e31, e12
            self.group6().xyz(),
            // e415, e425, e435, e321
            self.group5().with_w(self[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group4() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (right_anti_dual[e4235] * self_2[e1])
                    + (right_anti_dual[e4315] * self_2[e2])
                    + (right_anti_dual[e4125] * self_2[e3])
                    + (right_anti_dual[e3215] * self_2[e4])
                    + (right_anti_dual[e1234] * self_2[e5]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0(),
            // e5
            right_anti_dual[scalar] * self_2[e5],
            // e15, e25, e35, e45
            (Simd32x4::from(right_anti_dual[e5]) * self_2.group0()) - (Simd32x4::from(self_2[e5]) * right_anti_dual.group1()),
            // e41, e42, e43
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz()) - (Simd32x3::from(right_anti_dual[e4]) * self_2.group0().xyz()),
            // e23, e31, e12
            (right_anti_dual.group1().zxy() * self_2.group0().yzx()) - (right_anti_dual.group1().yzx() * self_2.group0().zxy()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (right_anti_dual[e15] * self_2[e4]) + (right_anti_dual[e41] * self_2[e5]),
                (right_anti_dual[e25] * self_2[e4]) + (right_anti_dual[e42] * self_2[e5]),
                (right_anti_dual[e35] * self_2[e4]) + (right_anti_dual[e43] * self_2[e5]),
                -(right_anti_dual[e31] * self_2[e2]) - (right_anti_dual[e12] * self_2[e3]),
            ]) - (self_2.group0().xyzx() * right_anti_dual.group3().www().with_w(right_anti_dual[e23])),
            // e423, e431, e412
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group5()) + (right_anti_dual.group4().yzx() * self_2.group0().zxy())
                - (right_anti_dual.group4().zxy() * self_2.group0().yzx()),
            // e235, e315, e125
            (Simd32x3::from(self_2[e5]) * right_anti_dual.group5()) + (right_anti_dual.group3().zxy() * self_2.group0().yzx())
                - (right_anti_dual.group3().yzx() * self_2.group0().zxy()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (right_anti_dual[e425] * self_2[e3]) + (right_anti_dual[e235] * self_2[e4]),
                (right_anti_dual[e435] * self_2[e1]) + (right_anti_dual[e315] * self_2[e4]),
                (right_anti_dual[e415] * self_2[e2]) + (right_anti_dual[e125] * self_2[e4]),
                -(right_anti_dual[e321] * self_2[e5]) - (right_anti_dual[e125] * self_2[e3]),
            ]) - (Simd32x4::from([self_2[e5], self_2[e5], self_2[e5], self_2[e1]]) * right_anti_dual.group7().with_w(right_anti_dual[e235]))
                - (self_2.group0().yzxy() * right_anti_dual.group6().zxy().with_w(right_anti_dual[e315])),
            // e1234
            (right_anti_dual[e321] * self_2[e4]) + (right_anti_dual[e423] * self_2[e1]) + (right_anti_dual[e431] * self_2[e2]) + (right_anti_dual[e412] * self_2[e3]),
        );
    }
}
impl std::ops::Div<support> for Plane {
    type Output = Dipole;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        6       20        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz(),
            // e23, e31, e12, e45
            ((right_anti_dual.group0().zxy() * self_2.group0().yzx()) - (right_anti_dual.group0().yzx() * self_2.group0().zxy())).with_w(right_anti_dual[e5] * self_2[e4]),
            // e15, e25, e35
            (Simd32x3::from(right_anti_dual[e5]) * self_2.group0().xyz()) - (Simd32x3::from(self_2[e5]) * right_anti_dual.group0().xyz()),
        );
    }
}
impl std::ops::Div<support> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return AntiScalar::from_groups(
            // e12345
            (self_2[e1] * right_anti_dual[e4235])
                + (self_2[e2] * right_anti_dual[e4315])
                + (self_2[e3] * right_anti_dual[e4125])
                + (self_2[e4] * right_anti_dual[e3215])
                + (self_2[e5] * right_anti_dual[e1234]),
        );
    }
}
impl std::ops::Div<support> for Sphere {
    type Output = Dipole;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        2        5        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd       10       24        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz()) - (Simd32x3::from(right_anti_dual[e4]) * self_2.group0().xyz()),
            // e23, e31, e12, e45
            (right_anti_dual.group0().zxy() * self_2.group0().yzx()).with_w(right_anti_dual[e5] * self_2[e4])
                - (right_anti_dual.group0().yzxw() * self_2.group0().zxy().with_w(self_2[e5])),
            // e15, e25, e35
            (Simd32x3::from(right_anti_dual[e5]) * self_2.group0().xyz()) - (Simd32x3::from(self_2[e5]) * right_anti_dual.group0().xyz()),
        );
    }
}
impl std::ops::Div<support> for VersorEven {
    type Output = VersorEven;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<support> for VersorEven {
    fn div_assign(&mut self, _rhs: support) {
        *self = self.support()
    }
}
impl Support for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        2        3        0
    //    simd4        3        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       24       59        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self_2[e2] * right_anti_dual[e43] * -1.0,
                self_2[e3] * right_anti_dual[e41] * -1.0,
                self_2[e1] * right_anti_dual[e42] * -1.0,
                (self_2[e3] * right_anti_dual[e4125]) + (self_2[e4] * right_anti_dual[e3215]) + (self_2[e5] * right_anti_dual[e1234]),
            ]) + (self_2.group0().zxyx() * right_anti_dual.group0().yzx().with_w(right_anti_dual[e4235]))
                + (self_2.group0().wwwy() * right_anti_dual.group1().xyz().with_w(right_anti_dual[e4315])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self_2[e4] * right_anti_dual[e15]) + (self_2[e5] * right_anti_dual[e41]),
                (self_2[e4] * right_anti_dual[e25]) + (self_2[e5] * right_anti_dual[e42]),
                (self_2[e4] * right_anti_dual[e35]) + (self_2[e5] * right_anti_dual[e43]),
                -(self_2[e2] * right_anti_dual[e31]) - (self_2[e3] * right_anti_dual[e12]),
            ]) - (self_2.group0().xyzx() * right_anti_dual.group1().wwwx()),
            // e235, e315, e125, e5
            ((Simd32x3::from(self_2[e5]) * right_anti_dual.group1().xyz()) + (self_2.group0().yzx() * right_anti_dual.group2().zxy())
                - (self_2.group0().zxy() * right_anti_dual.group2().yzx()))
            .with_w(self_2[e5] * right_anti_dual[scalar]),
            // e1, e2, e3, e4
            Simd32x4::from(right_anti_dual[scalar]) * self_2.group0(),
        );
    }
}
impl std::ops::Div<support> for VersorOdd {
    type Output = DipoleInversion;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        3        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       25       59        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x3::from(0.0).with_w(1.0), /* e5 */ 0.0);
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group3().xyz()) - (Simd32x3::from(right_anti_dual[e4]) * self_2.group0().xyz()),
            // e23, e31, e12, e45
            (self_2.group0().yzxw() * right_anti_dual.group3().zxy().with_w(right_anti_dual[e5]))
                - (self_2.group0().zxy() * right_anti_dual.group3().yzx()).with_w(self_2[e5] * right_anti_dual[e4]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self_2[e5] * right_anti_dual[e1] * -1.0,
                self_2[e5] * right_anti_dual[e2] * -1.0,
                self_2[e5] * right_anti_dual[e3] * -1.0,
                (self_2[e2] * right_anti_dual[e431]) + (self_2[e3] * right_anti_dual[e412]) + (self_2[e4] * right_anti_dual[e321]),
            ]) + (self_2.group0().xyzx() * right_anti_dual.group2().www().with_w(right_anti_dual[e423])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self_2[e3] * right_anti_dual[e425]) + (self_2[e4] * right_anti_dual[e235]),
                (self_2[e1] * right_anti_dual[e435]) + (self_2[e4] * right_anti_dual[e315]),
                (self_2[e2] * right_anti_dual[e415]) + (self_2[e4] * right_anti_dual[e125]),
                -(self_2[e3] * right_anti_dual[e125]) - (self_2[e5] * right_anti_dual[e321]),
            ]) - (Simd32x4::from([self_2[e5], self_2[e5], self_2[e5], right_anti_dual[e315]]) * right_anti_dual.group0().xyz().with_w(self_2[e2]))
                - (self_2.group0().yzxx() * right_anti_dual.group1().zxy().with_w(right_anti_dual[e235])),
        );
    }
}
