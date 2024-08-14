use crate::traits::AntiReverse;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl AntiReverse for AntiCircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for AntiDipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)));
    }
}
impl AntiReverse for AntiDualNum {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for AntiFlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return AntiFlatOrigin::from_groups(/* e321 */ (self[e321] * -1.0));
    }
}
impl AntiReverse for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)));
    }
}
impl AntiReverse for AntiFlector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e1, e2, e3, e5 */ self.group1());
    }
}
impl AntiReverse for AntiFlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
    }
}
impl AntiReverse for AntiLine {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return AntiLine::from_groups(
            // e23, e31, e12
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for AntiMotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for AntiPlane {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for AntiPlaneOnOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for AntiScalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for AntiSphereOnOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for AntiVersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for AntiVersorOddOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return AntiVersorOddOnOrigin::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(-1.0)), /* e4, e1, e2, e3 */ self.group1());
    }
}
impl AntiReverse for Circle {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
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
impl AntiReverse for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for Dipole {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return DipoleAtOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ (self.group0() * Simd32x4::from(-1.0)));
    }
}
impl AntiReverse for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn anti_reverse(self) -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for DualNum {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
    }
}
impl AntiReverse for FlatPoint {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(-1.0)));
    }
}
impl AntiReverse for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for Flector {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl AntiReverse for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return FlectorAtInfinity::from_groups(
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_reverse(self) -> Self {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
    }
}
impl AntiReverse for Horizon {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Infinity {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_reverse(self) -> Self {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiReverse for LineAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for LineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn anti_reverse(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (self.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
    }
}
impl AntiReverse for NullCircleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for NullDipoleAtOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn anti_reverse(self) -> Self {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiReverse for NullSphereAtOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for NullVersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return NullVersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for NullVersorOddAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_reverse(self) -> Self {
        return NullVersorOddAtOrigin::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiReverse for Origin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Plane {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for PlaneOnOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for RoundPoint {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for RoundPointAtOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Scalar {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for Sphere {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for SphereAtOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for SphereOnOrigin {
    fn anti_reverse(self) -> Self {
        return self;
    }
}
impl AntiReverse for VersorEven {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl AntiReverse for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiReverse for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl AntiReverse for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl AntiReverse for VersorOdd {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn anti_reverse(self) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
    }
}
impl AntiReverse for VersorOddAligningOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl AntiReverse for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn anti_reverse(self) -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl AntiReverse for VersorOddAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_reverse(self) -> Self {
        return VersorOddAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiReverse for VersorOddOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_reverse(self) -> Self {
        return VersorOddOnOrigin::from_groups(
            // e41, e42, e43, e45
            (self.group0() * Simd32x4::from(-1.0)),
            // e1234, e4235, e4315, e4125
            self.group1(),
        );
    }
}
impl AntiReverse for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn anti_reverse(self) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
