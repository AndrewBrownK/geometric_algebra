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
//   Median:         4      11       0
//  Average:         4      12       0
//  Maximum:        20      47       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       4       0
//   Median:         8      24       0
//  Average:         9      26       0
//  Maximum:        34      90       0
impl std::ops::Div<anti_support> for AntiCircleRotor {
    type Output = DipoleInversion;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        1        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       16       39        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * right_complement.group0().yzx()) - (right_dual.group0().yzx() * right_complement.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * right_complement[e3215],
                right_dual[e431] * right_complement[e3215],
                right_dual[e412] * right_complement[e3215],
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_dual.group1().wwwx() * right_complement.group0().xyzx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_complement[e3215]) * right_dual.group1().xyz()).with_w(0.0) + (right_dual.group2().yzx() * right_complement.group0().zxy()).with_w(0.0)
                - (right_dual.group2().zxy() * right_complement.group0().yzx()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * right_complement.group0(),
        );
    }
}
impl std::ops::Div<anti_support> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<anti_support> for AntiDipoleInversion {
    fn div_assign(&mut self, _rhs: anti_support) {
        *self = self.anti_support()
    }
}
impl AntiSupport for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       25        0
    //  no simd       17       51        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * right_complement.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_dual.group3().yzx() * right_complement.group0().zxy()) - (right_dual.group3().zxy() * right_complement.group0().yzx()))
                .with_w(right_dual[e1234] * right_complement[e3215] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_dual[e4235] * right_complement[e3215] * -1.0,
                right_dual[e4315] * right_complement[e3215] * -1.0,
                right_dual[e4125] * right_complement[e3215] * -1.0,
                (right_dual[e42] * right_complement[e4315]) + (right_dual[e43] * right_complement[e4125]),
            ]) + (right_complement.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * right_complement[e4315],
                right_dual[e23] * right_complement[e4125],
                right_dual[e31] * right_complement[e4235],
                -(right_dual[e25] * right_complement[e4315]) - (right_dual[e35] * right_complement[e4125]),
            ]) - (Simd32x4::from(right_complement[e3215]) * right_dual.group0().with_w(right_dual[e45]))
                - (right_complement.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15])),
        );
    }
}
impl std::ops::Div<anti_support> for AntiDualNum {
    type Output = Plane;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiDualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * Simd32x3::from(0.0).with_w(1.0));
    }
}
impl std::ops::Div<anti_support> for AntiFlatPoint {
    type Output = DualNum;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            -(right_dual[e15] * right_complement[e4235])
                - (right_dual[e25] * right_complement[e4315])
                - (right_dual[e35] * right_complement[e4125])
                - (right_dual[e45] * right_complement[e3215]),
            0.0,
        ]));
    }
}
impl std::ops::Div<anti_support> for AntiFlector {
    type Output = Motor;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        9       24        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((right_dual.group1().yzx() * right_complement.group0().zxy()) - (right_dual.group1().zxy() * right_complement.group0().yzx())).with_w(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                right_dual[e3215] * right_complement[e4235],
                right_dual[e3215] * right_complement[e4315],
                right_dual[e3215] * right_complement[e4125],
                -(right_dual[e25] * right_complement[e4315]) - (right_dual[e35] * right_complement[e4125]) - (right_dual[e45] * right_complement[e3215]),
            ]) - (right_complement.group0().wwwx() * right_dual.group1().xyz().with_w(right_dual[e15])),
        );
    }
}
impl std::ops::Div<anti_support> for AntiLine {
    type Output = FlatPoint;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiLine {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       18        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * right_complement[e3215]) + (right_dual[e315] * right_complement[e4125]),
                (right_dual[e425] * right_complement[e3215]) + (right_dual[e125] * right_complement[e4235]),
                (right_dual[e435] * right_complement[e3215]) + (right_dual[e235] * right_complement[e4315]),
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_complement.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
        );
    }
}
impl std::ops::Div<anti_support> for AntiMotor {
    type Output = Flector;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        8       24        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * right_complement[e3215]) + (right_dual[e315] * right_complement[e4125]),
                (right_dual[e425] * right_complement[e3215]) + (right_dual[e125] * right_complement[e4235]),
                (right_dual[e435] * right_complement[e3215]) + (right_dual[e235] * right_complement[e4315]),
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_complement.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e415])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * right_complement.group0(),
        );
    }
}
impl std::ops::Div<anti_support> for AntiPlane {
    type Output = Line;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for AntiPlane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        6       16        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
        return Line::from_groups(
            // e415, e425, e435
            (right_complement.group0().zxy() * right_dual.group0().yzx()) - (right_complement.group0().yzx() * right_dual.group0().zxy()),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * right_complement.group0().xyz()) - (Simd32x3::from(right_complement[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl std::ops::Div<anti_support> for Circle {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd       11       20        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2(),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * right_complement[e3215]) - (right_dual[e31] * right_complement[e4125]),
                -(right_dual[e42] * right_complement[e3215]) - (right_dual[e12] * right_complement[e4235]),
                -(right_dual[e43] * right_complement[e3215]) - (right_dual[e23] * right_complement[e4315]),
                (right_dual[e42] * right_complement[e4315]) + (right_dual[e43] * right_complement[e4125]),
            ]) + (right_complement.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * right_complement[e3215])
                - (right_dual[e15] * right_complement[e4235])
                - (right_dual[e25] * right_complement[e4315])
                - (right_dual[e35] * right_complement[e4125]),
        );
    }
}
impl std::ops::Div<anti_support> for CircleRotor {
    type Output = RoundPoint;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       15        0
    //  no simd       11       24        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * right_complement[e3215]) - (right_dual[e31] * right_complement[e4125]),
                -(right_dual[e42] * right_complement[e3215]) - (right_dual[e12] * right_complement[e4235]),
                -(right_dual[e43] * right_complement[e3215]) - (right_dual[e23] * right_complement[e4315]),
                (right_dual[e42] * right_complement[e4315]) + (right_dual[e43] * right_complement[e4125]),
            ]) + (right_complement.group0().yzxx() * right_dual.group1().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e45] * right_complement[e3215])
                - (right_dual[e15] * right_complement[e4235])
                - (right_dual[e25] * right_complement[e4315])
                - (right_dual[e35] * right_complement[e4125]),
        );
    }
}
impl std::ops::Div<anti_support> for Dipole {
    type Output = Dipole;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<anti_support> for Dipole {
    fn div_assign(&mut self, _rhs: anti_support) {
        *self = self.anti_support()
    }
}
impl AntiSupport for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        7        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       14       34        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
        return Dipole::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * right_complement.group0().yzx()) - (right_dual.group0().yzx() * right_complement.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * right_complement[e3215],
                right_dual[e431] * right_complement[e3215],
                right_dual[e412] * right_complement[e3215],
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_dual.group1().wwwx() * right_complement.group0().xyzx()),
            // e15, e25, e35
            (Simd32x3::from(right_complement[e3215]) * right_dual.group1().xyz()) + (right_dual.group2().yzx() * right_complement.group0().zxy())
                - (right_dual.group2().zxy() * right_complement.group0().yzx()),
        );
    }
}
impl std::ops::Div<anti_support> for DipoleInversion {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DipoleInversion {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        3        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        6       22        0
    //  no simd       17       46        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            (right_dual.group0().zxy() * right_complement.group0().yzx()) - (right_dual.group0().yzx() * right_complement.group0().zxy()),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_dual[e423] * right_complement[e3215],
                right_dual[e431] * right_complement[e3215],
                right_dual[e412] * right_complement[e3215],
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_dual.group1().wwwx() * right_complement.group0().xyzx()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                right_dual[e125] * right_complement[e4315] * -1.0,
                right_dual[e235] * right_complement[e4125] * -1.0,
                right_dual[e315] * right_complement[e4235] * -1.0,
                (right_dual[e2] * right_complement[e4315]) + (right_dual[e3] * right_complement[e4125]),
            ]) + (Simd32x4::from(right_complement[e3215]) * right_dual.group1().xyz().with_w(right_dual[e4]))
                + (right_complement.group0().zxyx() * right_dual.group2().yzx().with_w(right_dual[e1])),
        );
    }
}
impl std::ops::Div<anti_support> for DualNum {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for DualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_dual = AntiDualNum::from_groups(/* e3215, scalar */ self.group0() * Simd32x2::from(-1.0));
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            right_dual.group0().xx().with_zw(right_dual[e3215], 0.0)
                * Simd32x3::from(1.0).with_w(0.0)
                * Simd32x3::from(0.0).with_w(1.0).xyz().with_w(0.0)
                * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl std::ops::Div<anti_support> for FlatPoint {
    type Output = AntiLine;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for FlatPoint {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        3       16        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(right_dual[e321]) * right_complement.group0().xyz() * Simd32x3::from(-1.0),
            // e15, e25, e35
            (right_dual.group0().yzx() * right_complement.group0().zxy()) - (right_dual.group0().zxy() * right_complement.group0().yzx()),
        );
    }
}
impl std::ops::Div<anti_support> for Flector {
    type Output = AntiMotor;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        5       25        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([right_complement[e4235], right_complement[e4315], right_complement[e4125], 1.0])
                * right_dual
                    .group0()
                    .www()
                    .with_w((right_dual[e1] * right_complement[e4235]) + (right_dual[e2] * right_complement[e4315]) + (right_dual[e3] * right_complement[e4125]))
                * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e3215
            ((right_dual.group0().yzx() * right_complement.group0().zxy()) - (right_dual.group0().zxy() * right_complement.group0().yzx())).with_w(0.0),
        );
    }
}
impl std::ops::Div<anti_support> for Line {
    type Output = AntiPlane;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Line {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1());
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * right_complement[e4315],
                right_dual[e23] * right_complement[e4125],
                right_dual[e31] * right_complement[e4235],
                -(right_dual[e25] * right_complement[e4315]) - (right_dual[e35] * right_complement[e4125]),
            ]) - (right_complement.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl std::ops::Div<anti_support> for Motor {
    type Output = AntiFlector;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Motor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       29        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x3::from(1.0).with_w(0.0) * right_dual.group1().www().with_w(0.0) * right_complement.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_dual[e12] * right_complement[e4315],
                right_dual[e23] * right_complement[e4125],
                right_dual[e31] * right_complement[e4235],
                -(right_dual[e25] * right_complement[e4315]) - (right_dual[e35] * right_complement[e4125]),
            ]) - (right_complement.group0().zxyx() * right_dual.group0().yzx().with_w(right_dual[e15])),
        );
    }
}
impl std::ops::Div<anti_support> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<anti_support> for MultiVector {
    fn div_assign(&mut self, _rhs: anti_support) {
        *self = self.anti_support()
    }
}
impl AntiSupport for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd2        0        1        0
    //    simd3        4       12        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       20       47        0
    //  no simd       34       90        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = MultiVector::from_groups(
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (right_dual[e1] * right_complement[e4235])
                    + (right_dual[e2] * right_complement[e4315])
                    + (right_dual[e3] * right_complement[e4125])
                    + (right_dual[e4] * right_complement[e3215]),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                -(right_dual[e41] * right_complement[e3215]) - (right_dual[e31] * right_complement[e4125]),
                -(right_dual[e42] * right_complement[e3215]) - (right_dual[e12] * right_complement[e4235]),
                -(right_dual[e43] * right_complement[e3215]) - (right_dual[e23] * right_complement[e4315]),
                (right_dual[e42] * right_complement[e4315]) + (right_dual[e43] * right_complement[e4125]),
            ]) + (right_complement.group0().yzxx() * right_dual.group5().zxy().with_w(right_dual[e41])),
            // e5
            -(right_dual[e15] * right_complement[e4235])
                - (right_dual[e25] * right_complement[e4315])
                - (right_dual[e35] * right_complement[e4125])
                - (right_dual[e45] * right_complement[e3215]),
            // e15, e25, e35, e45
            Simd32x4::from([
                (right_dual[e415] * right_complement[e3215]) + (right_dual[e315] * right_complement[e4125]),
                (right_dual[e425] * right_complement[e3215]) + (right_dual[e125] * right_complement[e4235]),
                (right_dual[e435] * right_complement[e3215]) + (right_dual[e235] * right_complement[e4315]),
                -(right_dual[e425] * right_complement[e4315]) - (right_dual[e435] * right_complement[e4125]),
            ]) - (right_complement.group0().yzxx() * right_dual.group8().zxy().with_w(right_dual[e415])),
            // e41, e42, e43
            (right_dual.group7().zxy() * right_complement.group0().yzx()) - (right_dual.group7().yzx() * right_complement.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(right_complement[e3215]) * right_dual.group7()) - (Simd32x3::from(right_dual[e321]) * right_complement.group0().xyz()),
            // e415, e425, e435, e321
            ((right_dual.group9().yzx() * right_complement.group0().zxy()) - (right_dual.group9().zxy() * right_complement.group0().yzx()))
                .with_w(right_dual[e1234] * right_complement[e3215] * -1.0),
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * right_complement.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * right_complement.group0().xyz()) - (Simd32x3::from(right_complement[e3215]) * right_dual.group9().xyz()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * right_complement.group0(),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Div<anti_support> for Plane {
    type Output = Scalar;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
        return Scalar::from_groups(
            // scalar
            (right_dual[e1] * right_complement[e4235]) + (right_dual[e2] * right_complement[e4315]) + (right_dual[e3] * right_complement[e4125]),
        );
    }
}
impl std::ops::Div<anti_support> for RoundPoint {
    type Output = Circle;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        6       25        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        );
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * right_complement.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_complement.group0().zxy() * right_dual.group0().yzx()) - (right_complement.group0().yzx() * right_dual.group0().zxy()))
                .with_w(right_complement[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125
            (Simd32x3::from(right_dual[e3215]) * right_complement.group0().xyz()) - (Simd32x3::from(right_complement[e3215]) * right_dual.group0().xyz()),
        );
    }
}
impl std::ops::Div<anti_support> for Scalar {
    type Output = Plane;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * Simd32x3::from(0.0).with_w(1.0));
    }
}
impl std::ops::Div<anti_support> for Sphere {
    type Output = Scalar;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        );
        return Scalar::from_groups(
            // scalar
            (right_complement[e4235] * right_dual[e1])
                + (right_complement[e4315] * right_dual[e2])
                + (right_complement[e4125] * right_dual[e3])
                + (right_complement[e3215] * right_dual[e4]),
        );
    }
}
impl std::ops::Div<anti_support> for VersorEven {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl AntiSupport for VersorEven {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        1        4        0
    //    simd4        3        7        0
    // Totals...
    // yes simd        6       26        0
    //  no simd       17       55        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(right_dual[e1234]) * right_complement.group0().xyz() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            ((right_complement.group0().zxy() * right_dual.group3().yzx()) - (right_complement.group0().yzx() * right_dual.group3().zxy()))
                .with_w(right_complement[e3215] * right_dual[e1234] * -1.0),
            // e235, e315, e125, e4
            Simd32x4::from([
                right_complement[e3215] * right_dual[e4235] * -1.0,
                right_complement[e3215] * right_dual[e4315] * -1.0,
                right_complement[e3215] * right_dual[e4125] * -1.0,
                (right_complement[e4315] * right_dual[e42]) + (right_complement[e4125] * right_dual[e43]),
            ]) + (right_complement.group0().xyzx() * right_dual.group3().www().with_w(right_dual[e41])),
            // e1, e2, e3, e5
            Simd32x4::from([
                right_complement[e4315] * right_dual[e12],
                right_complement[e4125] * right_dual[e23],
                right_complement[e4235] * right_dual[e31],
                -(right_complement[e4125] * right_dual[e35]) - (right_complement[e3215] * right_dual[e45]),
            ]) - (right_complement.group0().zxyx() * right_dual.group1().yzx().with_w(right_dual[e15]))
                - (right_complement.group0().wwwy() * right_dual.group0().xyz().with_w(right_dual[e25])),
        );
    }
}
impl std::ops::Div<anti_support> for VersorOdd {
    type Output = VersorOdd;
    fn div(self, _rhs: anti_support) -> Self::Output {
        self.anti_support()
    }
}
impl std::ops::DivAssign<anti_support> for VersorOdd {
    fn div_assign(&mut self, _rhs: anti_support) {
        *self = self.anti_support()
    }
}
impl AntiSupport for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       14        0
    //    simd3        0        3        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        7       24        0
    //  no simd       19       51        0
    fn anti_support(self) -> Self::Output {
        use crate::elements::*;
        let right_complement = Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x3::from(0.0).with_w(1.0));
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                right_complement[e4125] * right_dual[e431] * -1.0,
                right_complement[e4235] * right_dual[e412] * -1.0,
                right_complement[e4315] * right_dual[e423] * -1.0,
                (right_complement[e4315] * right_dual[e2]) + (right_complement[e4125] * right_dual[e3]) + (right_complement[e3215] * right_dual[e4]),
            ]) + (right_complement.group0().yzxx() * right_dual.group0().zxy().with_w(right_dual[e1])),
            // e23, e31, e12, e45
            Simd32x4::from([
                right_complement[e3215] * right_dual[e423],
                right_complement[e3215] * right_dual[e431],
                right_complement[e3215] * right_dual[e412],
                -(right_complement[e4315] * right_dual[e425]) - (right_complement[e4125] * right_dual[e435]),
            ]) - (right_complement.group0().xyzx() * right_dual.group1().wwwx()),
            // e15, e25, e35, e1234
            (Simd32x3::from(right_complement[e3215]) * right_dual.group1().xyz()).with_w(0.0) + (right_complement.group0().zxy() * right_dual.group2().yzx()).with_w(0.0)
                - (right_complement.group0().yzx() * right_dual.group2().zxy()).with_w(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(right_dual[e12345]) * right_complement.group0(),
        );
    }
}
