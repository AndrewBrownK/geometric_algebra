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
//   Median:         0       4       0
//  Average:         0       3       0
//  Maximum:         0      16       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       3       0
//  Maximum:         0      16       0
impl ConformalConjugate for AntiCircleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl ConformalConjugate for AntiCircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl ConformalConjugate for AntiCircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl ConformalConjugate for AntiCircleRotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiDipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiDipoleInversionOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiDipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return AntiDipoleInversionOrthogonalOrigin::from_groups(
            // e423, e431, e412, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e415, e425, e435
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e235, e315, e125, e4
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl ConformalConjugate for AntiDipoleOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiDualNum {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiFlatOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl ConformalConjugate for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiFlectorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiLine::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiLineOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiMotorOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiMysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return AntiMysteryCircleRotor::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // scalar
            self[e31],
        );
    }
}
impl ConformalConjugate for AntiMysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return AntiMysteryDipoleInversion::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3
            self.group1(),
        );
    }
}
impl ConformalConjugate for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for AntiPlaneOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * -1.0));
    }
}
impl ConformalConjugate for AntiSphereOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for AntiVersorEvenOnOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn conformal_conjugate(self) -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn conformal_conjugate(self) -> Self {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e235, e315, e125
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn conformal_conjugate(self) -> Self {
        return CircleAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleRotorAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleRotorAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleRotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for CircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e415, e425, e435
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return DipoleAtOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), (self.group3()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleInversionAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleInversionAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleInversionAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl ConformalConjugate for DipoleInversionOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[0], (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleInversionOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
impl ConformalConjugate for DipoleOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DipoleOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35
            Simd32x3::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([self.group0()[0], (self.group0()[1] * -1.0)]));
    }
}
impl ConformalConjugate for FlatOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ (self[e45] * -1.0));
    }
}
impl ConformalConjugate for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for FlatPointAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return FlatPointAtInfinity::from_groups(
            // e15, e25, e35
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for FlectorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for FlectorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for Horizon {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ (self[e3215] * -1.0));
    }
}
impl ConformalConjugate for Infinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Infinity::from_groups(/* e5 */ (self[e5] * -1.0));
    }
}
impl ConformalConjugate for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn conformal_conjugate(self) -> Self {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e235, e315, e125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for LineAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return LineAtInfinity::from_groups(
            // e235, e315, e125
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for LineOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return LineOnOrigin::from_groups(
            // e415, e425, e435
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for MotorAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for MotorOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       16        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from([(self.group4()[0] * -1.0), (self.group4()[1] * -1.0), (self.group4()[2] * -1.0)]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group6()[0] * -1.0), (self.group6()[1] * -1.0), (self.group6()[2] * -1.0), self.group6()[3]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([(self.group8()[0] * -1.0), (self.group8()[1] * -1.0), (self.group8()[2] * -1.0)]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group9()[0], (self.group9()[1] * -1.0), (self.group9()[2] * -1.0), (self.group9()[3] * -1.0)]),
            // e3215
            (self[e45] * -1.0),
        );
    }
}
impl ConformalConjugate for MysteryCircle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return MysteryCircle::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl ConformalConjugate for MysteryCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return MysteryCircleRotor::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e12345
            (self[e425] * -1.0),
        );
    }
}
impl ConformalConjugate for MysteryDipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return MysteryDipole::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for MysteryDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MysteryDipoleInversion::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e4235, e4315, e4125
            Simd32x3::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for MysteryVersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl ConformalConjugate for MysteryVersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return MysteryVersorOdd::from_groups(
            // scalar, e4235, e4315, e4125
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for NullCircleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for NullDipoleAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for NullDipoleInversionAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for NullSphereAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for NullVersorEvenAtOrigin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for Origin {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            (self.group0()[0] * -1.0),
            (self.group0()[1] * -1.0),
            (self.group0()[2] * -1.0),
            (self.group0()[3] * -1.0),
        ]));
    }
}
impl ConformalConjugate for PlaneOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return PlaneOnOrigin::from_groups(
            // e4235, e4315, e4125
            Simd32x3::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
        );
    }
}
impl ConformalConjugate for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ self.group0(), /* e5 */ (self[e2] * -1.0));
    }
}
impl ConformalConjugate for RoundPointAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from([self.group0()[0], (self.group0()[1] * -1.0)]));
    }
}
impl ConformalConjugate for Scalar {
    fn conformal_conjugate(self) -> Self {
        return self;
    }
}
impl ConformalConjugate for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e1234
            self[e4315],
        );
    }
}
impl ConformalConjugate for SphereAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn conformal_conjugate(self) -> Self {
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from([(self.group0()[0] * -1.0), self.group0()[1]]));
    }
}
impl ConformalConjugate for SphereOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn conformal_conjugate(self) -> Self {
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl ConformalConjugate for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            self.group3(),
        );
    }
}
impl ConformalConjugate for VersorEvenAligningOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for VersorEvenAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for VersorEvenAtOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for VersorEvenOnOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e415, e425, e435, e4
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl ConformalConjugate for VersorEvenOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), (self.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            self.group2(),
        );
    }
}
impl ConformalConjugate for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), (self.group3()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn conformal_conjugate(self) -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), (self.group2()[3] * -1.0)]),
        );
    }
}
impl ConformalConjugate for VersorOddOrthogonalOrigin {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn conformal_conjugate(self) -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group2()[3]]),
        );
    }
}
