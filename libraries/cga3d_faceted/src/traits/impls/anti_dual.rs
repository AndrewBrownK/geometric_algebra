// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 87
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       2       0
//  Maximum:         0      12       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       3       0
//  Maximum:         0      16       0
impl AntiDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for AntiCircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for AntiCircleRotorAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for AntiCircleRotorAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiDipoleInversionOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group1()[0] * -1.0), self.group1()[1], self.group1()[2], self.group1()[3]]),
        );
    }
}
impl AntiDual for AntiDipoleInversionOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiDualNum {
    type Output = AntiDualNum;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for AntiFlatOrigin {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ (self[e321] * -1.0));
    }
}
impl AntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
    }
}
impl AntiDual for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_dual(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e12345] * -1.0));
    }
}
impl AntiDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiVersorEvenOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35
            self.group2(),
        );
    }
}
impl AntiDual for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl AntiDual for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl AntiDual for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for CircleRotorAligningOriginAtInfinity {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for CircleRotorAtInfinity {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12
            self.group1(),
        );
    }
}
impl AntiDual for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_dual(self) -> Self::Output {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), self.group3()[3]]),
        );
    }
}
impl AntiDual for DipoleInversionAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn anti_dual(self) -> Self::Output {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for DipoleInversionOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e4, e1, e2, e3
            Simd32x4::from([self.group1()[0], (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for DipoleInversionOrthogonalOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_dual(self) -> Self::Output {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_dual(self) -> Self::Output {
        return AntiDualNum::from_groups(/* e3215, scalar */ (self.group0() * Simd32x2::from(-1.0)));
    }
}
impl AntiDual for FlatOrigin {
    type Output = FlatOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_dual(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_dual(self) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for Horizon {
    type Output = Horizon;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Infinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ (self[e5] * -1.0));
    }
}
impl AntiDual for Line {
    type Output = Line;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for LineAtInfinity {
    type Output = LineAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for LineOnOrigin {
    type Output = LineOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(
            // e15, e25, e35, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[1] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group9()[1] * -1.0), (self.group9()[2] * -1.0), (self.group9()[3] * -1.0), self.group9()[0]]),
            // e5
            self[e45],
            // e41, e42, e43, e45
            Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], (self.group6()[3] * -1.0)]),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group5()[0] * -1.0), (self.group5()[1] * -1.0), (self.group5()[2] * -1.0), self.group3()[3]]),
            // e423, e431, e412
            (Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group4() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group1()[3] * -1.0), self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e3215
            (self[e1] * -1.0),
        );
    }
}
impl AntiDual for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_dual(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for NullDipoleInversionAtOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return NullVersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for NullVersorEvenAtOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        return NullDipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for Origin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ (self[e4] * -1.0));
    }
}
impl AntiDual for Plane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self[e2] * -1.0)]),
            // e1234
            (self.group0()[3] * -1.0),
        );
    }
}
impl AntiDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_dual(self) -> Self::Output {
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ (swizzle!(self.group0(), 1, 0) * Simd32x2::from(-1.0)));
    }
}
impl AntiDual for Scalar {
    type Output = Scalar;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self[e4315]]),
            // e5
            self.group0()[3],
        );
    }
}
impl AntiDual for SphereAtOrigin {
    type Output = RoundPointAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from([self.group0()[1], self.group0()[0]]));
    }
}
impl AntiDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_dual(self) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group3()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorEvenAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group2()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (self.group2()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorEvenAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorEvenOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dual(self) -> Self::Output {
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorEvenOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_dual(self) -> Self::Output {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group2()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn anti_dual(self) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn anti_dual(self) -> Self::Output {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[0], (self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiDual for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn anti_dual(self) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group2()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group1()[3]]),
        );
    }
}
