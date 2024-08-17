use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl std::ops::Add<AntiCircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            other.group1(),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group2()[3],
            ]),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group2()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
            // e23, e31, e12, e45
            other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3]]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1]]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] + other.group0()[0])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e23, e31, e12
            other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e23, e31, e12
            other.group0(),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotor> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversion> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            (self.group1() + other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group0()[3] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            (self.group1() + other.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[3] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            other.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[3] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() + other.group0()),
            // e15, e25, e35, e1234
            (self.group1() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() + other.group0()),
            // e15, e25, e35, e1234
            (self.group1() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[0])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() + other.group0()),
            // e23, e31, e12
            other.group1(),
            // e15, e25, e35, e1234
            (self.group1() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e45]]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e3215])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Horizon> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e3215])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Infinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Line> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group3()[0]),
                (self.group0()[1] + other.group3()[1]),
                (self.group0()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
            // e15, e25, e35
            (other.group4() + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group1()[3] + other.group9()[0]), other.group9()[1], other.group9()[2], other.group9()[3]]),
            // e3215
            (self.group0()[3] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e1234])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e1234])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[1])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for DipoleInversionAtOrigin {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        let addition = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[1])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            (self.group1() + other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group0()[3] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e23, e31, e12, e45
            other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[3] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[3] + other.group1()[3])]),
            // e15, e25, e35, e1234
            (self.group1() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        7       12        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       19       26        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        9       22        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       19        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       13       14        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for DipoleInversionAtOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        9       22        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       12        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for DipoleInversionAtOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        2        3        0
    // no simd        4        6        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        2        2        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        9       12        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        3        4        0
    // no simd        9       12        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       12        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       41        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       31       52        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       10       24        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for DipoleInversionAtOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       19       26        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       13       14        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        9       22        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       13       14        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       19       26        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<DipoleAtOrigin> for DipoleInversionAtOrigin {
    fn from(dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35], 0.0]),
        );
    }
}

impl From<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35], 0.0]),
        );
    }
}

impl From<FlectorAtInfinity> for DipoleInversionAtOrigin {
    fn from(flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35], 0.0]),
        );
    }
}

impl From<Horizon> for DipoleInversionAtOrigin {
    fn from(horizon: Horizon) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, horizon[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    fn from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([null_dipole_inversion_at_origin[e41], null_dipole_inversion_at_origin[e42], null_dipole_inversion_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_dipole_inversion_at_origin[e1234]]),
        );
    }
}

impl From<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    fn from(null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_sphere_at_origin[e1234]]),
        );
    }
}

impl From<SphereAtOrigin> for DipoleInversionAtOrigin {
    fn from(sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e1234]]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       32       48        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4       13       15        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       72       93        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       64       81        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       63        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       16        0
    //    simd4       10       14        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       51       72        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       40       56        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       50       66        0
    //  no simd      104      120        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       37       52        0
    //  no simd       73       88        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       45        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       42       56        0
    //  no simd       75       89        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       16       40        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       24        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       30        0
    //  no simd       16       36        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       48       64        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       24       32        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       32       48        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       48       64        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       24       32        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       16       32        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       13       30        0
    //  no simd       16       39        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       48       64        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       34       51        0
    //  no simd       64       81        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       56       73        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       33        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       57        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       40       48        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       32       48        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       28       45        0
    //  no simd       40       60        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       72       89        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       37       54        0
    //  no simd       64       81        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       56        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       33        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       48       65        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       39        0
    //  no simd       40       63        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       37        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       34       49        0
    //  no simd       64       85        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       40       61        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       40       60        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       47       63        0
    //  no simd      104      120        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       72       89        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       44        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       41       49        0
    //  no simd       56       64        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       48       64        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       49        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       45       59        0
    //  no simd       75       89        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       25        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       16       37        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       56       73        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       16        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       16       41        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       16       24        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       48       64        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       20        0
    //  no simd       24       32        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        8        0
    // no simd       24       32        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       15        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       32       48        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       24       32        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       24       32        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       66       90        0
    //    simd2        7        8        0
    //    simd3       24       26        0
    //    simd4       18       18        0
    // Totals...
    // yes simd      115      142        0
    //  no simd      224      256        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       16       24        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        8        0
    // no simd       24       32        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       16       43        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       12       29        0
    //  no simd       24       47        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for DipoleInversionAtOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       14        0
    //    simd4        6       10        0
    // Totals...
    // yes simd        7       24        0
    //  no simd       25       54        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       16       36        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       49       65        0
    //  no simd      112      128        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       81       96        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       84       96        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       20       28        0
    //  no simd       56       64        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       41       54        0
    //  no simd       83       96        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       49       65        0
    //  no simd      112      128        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       40        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       35       55        0
    //  no simd       80      100        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       81       96        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Not for DipoleInversionAtOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn not(self) -> Self::Output {
        let dual = VersorEvenAtOrigin::from_groups(
            // e423, e431, e412, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group0()[3]]),
        );
        return dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        5        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e23, e31, e12, e45
            (other.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
            // e23, e31, e12, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group2()[3] * -1.0)]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] - other.group0()[0])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group1()[3]),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        6        4        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            (other.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        1        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        8        7        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            (other.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            (self.group1() - other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group0()[3] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (self.group1() - other.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4        7        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() - other.group0()),
            // e15, e25, e35, e1234
            (self.group1() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() - other.group0()),
            // e15, e25, e35, e1234
            (self.group1() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[0])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group1()[3] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        8        3        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            (self.group0() - other.group0()),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            (self.group1() - other.group2()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e45] * -1.0)]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[3] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e3215])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Horizon> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e3215])]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Infinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other[e5] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        5        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       24        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group3()[0]),
                (self.group0()[1] - other.group3()[1]),
                (self.group0()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
            // e15, e25, e35
            (-other.group4() + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                (self.group1()[3] - other.group9()[0]),
                (other.group9()[1] * -1.0),
                (other.group9()[2] * -1.0),
                (other.group9()[3] * -1.0),
            ]),
            // e3215
            (self.group0()[3] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e1234])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e1234])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[scalar] * -1.0)]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e15, e25, e35, e1234
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for DipoleInversionAtOrigin {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        let subtraction = DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[1])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for DipoleInversionAtOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            self.group0()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        8        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, e45
            (other.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            (self.group1() - other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group0()[3] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for DipoleInversionAtOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        5        0
    //  no simd        4        8        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e23, e31, e12, e45
            (other.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[3] - other.group1()[3]),
            ]),
            // e15, e25, e35, e1234
            (self.group1() - other.group2()),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e41],
                anti_circle_rotor_aligning_origin[e42],
                anti_circle_rotor_aligning_origin[e43],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e15],
                anti_circle_rotor_aligning_origin[e25],
                anti_circle_rotor_aligning_origin[e35],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e15],
                anti_circle_rotor_aligning_origin_at_infinity[e25],
                anti_circle_rotor_aligning_origin_at_infinity[e35],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor_at_infinity[e15], anti_circle_rotor_at_infinity[e25], anti_circle_rotor_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([anti_circle_rotor_on_origin[e41], anti_circle_rotor_on_origin[e42], anti_circle_rotor_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDualNum> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiLine> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiMotor> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([anti_versor_even_on_origin[e41], anti_versor_even_on_origin[e42], anti_versor_even_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_even_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<Dipole> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversion> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43], dipole_inversion[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], dipole_inversion[e1234]]),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                dipole_inversion_aligning_origin[e41],
                dipole_inversion_aligning_origin[e42],
                dipole_inversion_aligning_origin[e43],
                dipole_inversion_aligning_origin[e3215],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                dipole_inversion_aligning_origin[e15],
                dipole_inversion_aligning_origin[e25],
                dipole_inversion_aligning_origin[e35],
                dipole_inversion_aligning_origin[e1234],
            ]),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_inversion_at_infinity[e15], dipole_inversion_at_infinity[e25], dipole_inversion_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_inversion_on_origin[e41], dipole_inversion_on_origin[e42], dipole_inversion_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e41],
                dipole_inversion_orthogonal_origin[e42],
                dipole_inversion_orthogonal_origin[e43],
                dipole_inversion_orthogonal_origin[e3215],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
                dipole_inversion_orthogonal_origin[e1234],
            ]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35], 0.0]),
        ));
    }
}

impl TryFrom<FlatPoint> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
        ));
    }
}

impl TryFrom<Flector> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
        ));
    }
}

impl TryFrom<Plane> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(plane: Plane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Plane do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, plane[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Sphere> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOdd> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e1234]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for DipoleInversionAtOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into DipoleInversionAtOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([
                versor_odd_orthogonal_origin[e41],
                versor_odd_orthogonal_origin[e42],
                versor_odd_orthogonal_origin[e43],
                versor_odd_orthogonal_origin[e3215],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
                versor_odd_orthogonal_origin[e1234],
            ]),
        ));
    }
}
