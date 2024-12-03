// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 95
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       1       0
//  Maximum:         0       8       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       5       0
//  Maximum:         0      22       0
impl std::ops::Div<right_anti_dual> for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiCircleRotor {
    type Output = CircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e12345
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleInversion {
    type Output = DipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234, e4235, e4315, e4125
            self.group1() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiDualNum {
    type Output = DualNum;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiDualNum {
    type Output = DualNum;
    fn right_anti_dual(self) -> Self::Output {
        return DualNum::from_groups(/* e4, e12345 */ self.group0());
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlatOrigin {
    type Output = FlatOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatOrigin {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ self[e321] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlatPoint {
    type Output = FlatPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlector {
    type Output = Flector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiLine {
    type Output = Line;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for AntiMotor {
    type Output = Motor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMysteryCircleRotor {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e12345
            self[scalar],
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiMysteryDipoleInversion {
    type Output = MysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125
            self.group1(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for AntiPlane {
    type Output = Plane;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ self.group0());
    }
}
impl std::ops::Div<right_anti_dual> for AntiScalar {
    type Output = Scalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for Circle {
    type Output = Dipole;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group2(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAligningOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1(), /* e15, e25, e35 */ self.group2());
    }
}
impl std::ops::Div<right_anti_dual> for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleAtOrigin {
    type Output = DipoleAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return DipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Div<right_anti_dual> for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ self.group0(), /* e23, e31, e12 */ self.group1());
    }
}
impl std::ops::Div<right_anti_dual> for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotor {
    type Output = AntiCircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12
            self.group1(),
        );
    }
}
impl std::ops::Div<right_anti_dual> for Dipole {
    type Output = Circle;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn right_anti_dual(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversion {
    type Output = AntiDipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group3() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().truncate_to_3().extend_to_4(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group2().truncate_to_3().extend_to_4(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125
            self.group1() * Simd32x3::from(-1.0),
            // e1, e2, e3, e5
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0().truncate_to_3().extend_to_4(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group1().truncate_to_3().extend_to_4(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4, e1, e2, e3
            self.group1() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e4
            self.group2() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn right_anti_dual(self) -> Self::Output {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for DualNum {
    type Output = AntiDualNum;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiDualNum::from_groups(/* e1234, scalar */ self.group0() * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatOrigin {
    type Output = AntiFlatOrigin;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiFlatOrigin::from_groups(/* e321 */ self[e45]);
    }
}
impl std::ops::Div<right_anti_dual> for FlatPoint {
    type Output = AntiFlatPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for Flector {
    type Output = AntiFlector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e5
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for Horizon {
    type Output = Infinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Horizon {
    type Output = Infinity;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Infinity::from_groups(/* e5 */ self[e3215]);
    }
}
impl std::ops::Div<right_anti_dual> for Infinity {
    type Output = Horizon;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Infinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ self[e5] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for Line {
    type Output = AntiLine;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Line {
    type Output = AntiLine;
    fn right_anti_dual(self) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Div<right_anti_dual> for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for LineAtInfinity {
    type Output = FlatPointAtInfinity;
    fn right_anti_dual(self) -> Self::Output {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ self.group0());
    }
}
impl std::ops::Div<right_anti_dual> for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for LineOnOrigin {
    type Output = AntiLineOnOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ self.group0());
    }
}
impl std::ops::Div<right_anti_dual> for Motor {
    type Output = AntiMotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for MultiVector {
    type Output = MultiVector;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl std::ops::DivAssign<right_anti_dual> for MultiVector {
    fn div_assign(&mut self, _rhs: right_anti_dual) {
        *self = self.right_anti_dual()
    }
}
impl RightAntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            crate::swizzle!(self.group0(), 1, 0) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            crate::swizzle!(self.group9(), 1, 2, 3, 0) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
            // e41, e42, e43, e45
            self.group7().extend_to_4(self[e321] * -1.0),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            self.group6().truncate_to_3(),
            // e415, e425, e435, e321
            self.group5().extend_to_4(self[e45]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412
            self.group3().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group4() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            crate::swizzle!(self.group1(), 3, 0, 1, 2) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e3215
            self[e5] * -1.0,
        );
    }
}
impl std::ops::Div<right_anti_dual> for MysteryCircle {
    type Output = MysteryDipole;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryCircle {
    type Output = MysteryDipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return MysteryDipole::from_groups(/* e23, e31, e12, e45 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryCircleRotor {
    type Output = AntiMysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // scalar
            self[e12345] * -1.0,
        );
    }
}
impl std::ops::Div<right_anti_dual> for MysteryDipole {
    type Output = MysteryCircle;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryDipole {
    type Output = MysteryCircle;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return MysteryCircle::from_groups(/* e415, e425, e435, e321 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryDipoleInversion {
    type Output = AntiMysteryDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Div<right_anti_dual> for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryVersorEven {
    type Output = MysteryVersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            self.group0() * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for MysteryVersorOdd {
    type Output = MysteryVersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            self.group0() * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullCircleAtOrigin {
    type Output = NullDipoleAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ self.group0());
    }
}
impl std::ops::Div<right_anti_dual> for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for NullSphereAtOrigin {
    type Output = Origin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullSphereAtOrigin {
    type Output = Origin;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234]);
    }
}
impl std::ops::Div<right_anti_dual> for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return NullDipoleInversionAtOrigin::from_groups(/* e41, e42, e43, e1234 */ self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for Origin {
    type Output = NullSphereAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ self[e4] * -1.0);
    }
}
impl std::ops::Div<right_anti_dual> for Plane {
    type Output = AntiPlane;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ self.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for RoundPoint {
    type Output = Sphere;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            self.group0().truncate_to_3().extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1234
            self[e4] * -1.0,
        );
    }
}
impl std::ops::Div<right_anti_dual> for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn right_anti_dual(self) -> Self::Output {
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ crate::swizzle!(self.group0(), 1, 0) * Simd32x2::from(-1.0));
    }
}
impl std::ops::Div<right_anti_dual> for Scalar {
    type Output = AntiScalar;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Scalar {
    type Output = AntiScalar;
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar]);
    }
}
impl std::ops::Div<right_anti_dual> for Sphere {
    type Output = RoundPoint;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            self.group0().truncate_to_3().extend_to_4(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e5
            self[e3215],
        );
    }
}
impl std::ops::Div<right_anti_dual> for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn right_anti_dual(self) -> Self::Output {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ crate::swizzle!(self.group0(), 1, 0));
    }
}
impl std::ops::Div<right_anti_dual> for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]));
    }
}
impl std::ops::Div<right_anti_dual> for VersorEven {
    type Output = VersorOdd;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().truncate_to_3().extend_to_4(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group3().truncate_to_3().extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e3215
            self.group1().truncate_to_3().extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2().truncate_to_3().extend_to_4(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self[e12345], self[e235], self[e315], self[e125]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            crate::swizzle!(self.group0(), 1, 2, 3, _).extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0().truncate_to_3().extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().truncate_to_3().extend_to_4(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn right_anti_dual(self) -> Self::Output {
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e1234
            self.group1() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0() * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group1().truncate_to_3().extend_to_4(self[e4]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group2().truncate_to_3().extend_to_4(self[e5]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorOdd {
    type Output = VersorEven;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().truncate_to_3().extend_to_4(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e1, e2, e3, e4
            self.group3().truncate_to_3().extend_to_4(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self[scalar], self[e4235], self[e4315], self[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e415, e425, e435, e321
            self.group1() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            crate::swizzle!(self.group0(), 1, 2, 3, _).extend_to_4(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Div<right_anti_dual> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    fn div(self, _rhs: right_anti_dual) -> Self::Output {
        self.right_anti_dual()
    }
}
impl RightAntiDual for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn right_anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e415, e425, e435, e4
            self.group1().truncate_to_3().extend_to_4(self[e1234]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e235, e315, e125, e5
            self.group2().truncate_to_3().extend_to_4(self[e3215]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
