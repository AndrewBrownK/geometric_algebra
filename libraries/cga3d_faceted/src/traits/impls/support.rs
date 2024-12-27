// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 76
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         0       2       0
//  Maximum:         0      15       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       5       0
//  Average:         0       7       0
//  Maximum:         0      40       0
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotor {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
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
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ right_anti_dual.group2().xyz().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[scalar]]).xyz(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotorAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[scalar]]).xyz(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiCircleRotorAtInfinity {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiCircleRotorAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ right_anti_dual.group1().xyz().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiDipoleInversion {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDipoleInversion {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group2().xyz(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiDipoleInversionOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().with_w(right_anti_dual[e3215]),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group2().xyz(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiFlatPoint {
    type Output = LineOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlatPoint {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0().xyz());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiFlector {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiFlector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ right_anti_dual.group0().xyz().with_w(right_anti_dual[e3215]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiLine {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiLine {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiMotor {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiMotor {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiMysteryCircleRotor {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiMysteryCircleRotor {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e45]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiMysteryDipoleInversion {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiMysteryDipoleInversion {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiPlane {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0().xyz().with_w(self[e5] * -1.0)[3]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for AntiScalar {
    type Output = Origin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Circle {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Circle {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz(),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group2(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleAligningOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleAligningOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual =
            DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1(), /* e15, e25, e35 */ self.group2());
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1(),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group2(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleAtInfinity {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleAtInfinity {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz(),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleAtOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleAtOrigin {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleOnOrigin {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group1());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleOrthogonalOrigin {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group1());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotor {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotor {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1().xyz()).with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotorAligningOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotorAligningOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group1()).with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotorAligningOriginAtInfinity {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotorAligningOriginAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group0()).with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotorAtInfinity {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotorAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz()).with_w(0.0),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
        );
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ right_anti_dual.group1().with_w(right_anti_dual[scalar]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Dipole {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Dipole {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
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
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ right_anti_dual.group2().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleAligningOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleAligningOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ right_anti_dual.group1().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleAtInfinity {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ right_anti_dual.group1().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversion {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversion {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       23        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversionAligningOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversionAligningOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().xyz().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2().xyz().with_w(right_anti_dual[e5]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversionAtInfinity {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversionAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversionAtOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversionOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        );
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ right_anti_dual.group1().yzw().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleInversionOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([right_anti_dual[e5], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleOnOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleOnOrigin {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e45]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DipoleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DipoleOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for DualNum {
    type Output = Origin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlatOrigin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlatOrigin {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e45]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlatPoint {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlatPoint {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0, self[e45]]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlatPointAtInfinity {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlatPointAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from([self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Flector {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().wxyz(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlectorAtInfinity {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlectorAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for FlectorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for FlectorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e45]]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Horizon {
    type Output = FlatOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Horizon {
    type Output = FlatOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e3215]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Infinity {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Infinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Line {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Line {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1());
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group0(),
            // e415, e425, e435
            Simd32x3::from(self_2[e4]) * right_anti_dual.group1(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for LineAtInfinity {
    type Output = LineOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for LineAtInfinity {
    type Output = LineOnOrigin;
    fn support(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for LineOnOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for LineOnOrigin {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Motor {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Motor {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[scalar]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MotorAtInfinity {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MotorAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0().xyz().with_w(self[e5] * -1.0));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0().xyz().with_w(self[e12345] * -1.0));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl std::ops::DivAssign<SupportPrefixOrPostfix> for MultiVector {
    fn div_assign(&mut self, _rhs: SupportPrefixOrPostfix) {
        *self = self.support()
    }
}
impl Support for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0       15        0
    //  no simd        0       40        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = MultiVector::from_groups(
            // scalar, e12345
            self.group0().yx() * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            self.group9().yzwx() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
            // e41, e42, e43, e45
            self.group7().with_w(self[e321] * -1.0),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            self.group6().xyz(),
            // e415, e425, e435, e321
            self.group5().with_w(self[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group3().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group4() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group1().wxyz() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e3215
            self[e5] * -1.0,
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, right_anti_dual[e3215] * self_2[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(right_anti_dual[scalar] * self_2[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[e5]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group4()).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(self_2[e4]) * right_anti_dual.group5(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryCircle {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryCircle {
    type Output = NullCircleAtOrigin;
    fn support(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0().xyz());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryCircleRotor {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryCircleRotor {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // scalar
            self[e12345] * -1.0,
        );
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ right_anti_dual.group0().xyz().with_w(right_anti_dual[scalar]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryDipole {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryDipole {
    type Output = NullSphereAtOrigin;
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e45]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryDipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryDipoleInversion {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        );
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ right_anti_dual.group1().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryVersorEven {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryVersorEven {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ right_anti_dual.group1().xyz().with_w(right_anti_dual[scalar]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for MysteryVersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for MysteryVersorOdd {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ right_anti_dual.group0().yzw().with_w(right_anti_dual[e321]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Plane {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Plane {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e3215]]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for PlaneOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for PlaneOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for RoundPoint {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0().xyz().with_w(self[e5] * -1.0)[3]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for RoundPointAtOrigin {
    type Output = AntiScalar;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for RoundPointAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for Sphere {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for Sphere {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().xyz().with_w(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            (Simd32x3::from(self_2[e4]) * right_anti_dual.group0().xyz()).with_w(self_2[e4] * right_anti_dual[e5]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for SphereAtOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for SphereAtOrigin {
    type Output = FlatOrigin;
    fn support(self) -> Self::Output {
        return FlatOrigin::from_groups(/* e45 */ self.group0().yx()[1]);
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for SphereOnOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for SphereOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x4::from([self[e4235] * -1.0, self[e4315] * -1.0, self[e4125] * -1.0, self[e1234]]).xyz());
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEven {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEven {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2().xyz().with_w(right_anti_dual[scalar]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEvenAligningOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEvenAligningOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e3215
            self.group1().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1(),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group2().xyz().with_w(right_anti_dual[scalar]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEvenAtInfinity {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEvenAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self[e12345], self[e235], self[e315], self[e125]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group0().yzw().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self_2[e4]) * right_anti_dual.group1().xyz().with_w(right_anti_dual[e3215]),
            // e415, e425, e435, e4
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().yzwx(),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEvenAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ right_anti_dual.group1().xyz().with_w(right_anti_dual[e3215]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEvenOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ right_anti_dual.group1().xyz().with_w(right_anti_dual[scalar]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorEvenOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().xyz().with_w(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group2().xyz().with_w(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ right_anti_dual.group1().xyz().with_w(right_anti_dual[e3215]));
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorOdd {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorOdd {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
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
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group3().xyz().with_w(right_anti_dual[e5]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        let right_anti_dual = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self[scalar], self[e4235], self[e4315], self[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group0().yzw().with_w(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        let self_2 = Origin::from_groups(/* e4 */ 1.0);
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(self_2[e4]) * right_anti_dual.group0().yzw().with_w(right_anti_dual[e5]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(self_2[e4]) * Simd32x4::from([right_anti_dual[e321], right_anti_dual[e235], right_anti_dual[e315], right_anti_dual[e125]]),
        );
    }
}
impl std::ops::Div<SupportPrefixOrPostfix> for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: SupportPrefixOrPostfix) -> Self::Output {
        self.support()
    }
}
impl Support for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn support(self) -> Self::Output {
        use crate::elements::*;
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([self[e3215], self[e15] * -1.0, self[e25] * -1.0, self[e35] * -1.0]));
    }
}
