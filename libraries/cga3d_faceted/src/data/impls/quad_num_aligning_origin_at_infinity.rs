use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 443
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         0       3       0
//  Maximum:        22      50       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         0       6       0
//  Maximum:        32      68       0
impl std::ops::Add<AntiCircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group3()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[0] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = QuadNumAtInfinity::from_groups(/* e5, e321, e12345 */ Simd32x3::from([self.group0()[0], other[e321], self.group0()[1]]));
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e31], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[2], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[2], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for QuadNumAligningOriginAtInfinity {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]));
        *self = addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    fn add(self, other: Circle) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group2()[3])]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group2()[3])]),
            // e415, e425, e435, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] + other.group1()[3]), 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e415, e425, e435, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([(self.group0()[0] + other[e5]), self.group0()[1]]));
        return addition;
    }
}
impl std::ops::AddAssign<Infinity> for QuadNumAligningOriginAtInfinity {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([(self.group0()[0] + other[e5]), self.group0()[1]]));
        *self = addition;
    }
}
impl std::ops::Add<Line> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    fn add(self, other: Line) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[0] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], (other.group0()[1] + self.group0()[1])]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            (self.group0()[0] + other[e1]),
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircle> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: MysteryCircle) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] + other[e425]), 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipole> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryQuadNum) -> Self::Output {
        let addition = QuadNumAtInfinity::from_groups(
            // e5, e321, e12345
            Simd32x3::from([self.group0()[0], other.group0()[0], (other.group0()[1] + self.group0()[1])]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] + other.group0()[0]), other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = QuadNumAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([other[e4], self.group0()[0], self.group0()[1]]));
        return addition;
    }
}
impl std::ops::Add<Plane> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<QuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: QuadNum) -> Self::Output {
        let addition = QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            other.group0()[0],
            (self.group0()[0] + other.group0()[1]),
            other.group0()[2],
            (self.group0()[1] + other.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: QuadNumAligningOrigin) -> Self::Output {
        let addition = QuadNumAligningOrigin::from_groups(
            // e4, e5, e12345
            Simd32x3::from([other.group0()[0], (self.group0()[0] + other.group0()[1]), (self.group0()[1] + other.group0()[2])]),
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ (other.group0() + self.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    fn add_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        let addition = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ (other.group0() + self.group0()));
        *self = addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: QuadNumAtInfinity) -> Self::Output {
        let addition = QuadNumAtInfinity::from_groups(
            // e5, e321, e12345
            Simd32x3::from([(self.group0()[0] + other.group0()[0]), other.group0()[1], (self.group0()[1] + other.group0()[2])]),
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: QuadNumOnOrigin) -> Self::Output {
        let addition = QuadNumAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([other.group0()[0], self.group0()[0], (self.group0()[1] + other.group0()[1])]));
        return addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let addition = QuadNum::from_groups(
            // e4, e5, e321, e12345
            Simd32x4::from([other.group0()[0], (self.group0()[0] + other.group0()[1]), other.group0()[2], self.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] + other[e2])]),
            // e1, e2, e3, e4
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = QuadNumAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([other.group0()[0], (self.group0()[0] + other.group0()[1]), self.group0()[1]]));
        return addition;
    }
}
impl std::ops::Add<Scalar> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[1], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group2()[3])]),
            // e1, e2, e3, e4
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e415, e425, e435, e4
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] + other.group0()[0]), other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[0] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e415, e425, e435, e4
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[0] + other.group1()[3])]),
            // e1, e2, e3, e4
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for QuadNumAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNum> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNumOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for QuadNumAligningOriginAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for QuadNumAligningOriginAtInfinity {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       16        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        1       19        0
    //  no simd        1       25        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for QuadNumAligningOriginAtInfinity {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for QuadNumAligningOriginAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for QuadNumAligningOriginAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiScalar> for QuadNumAligningOriginAtInfinity {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, anti_scalar[e12345]]));
    }
}

impl From<Infinity> for QuadNumAligningOriginAtInfinity {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([infinity[e5], 0.0]));
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        3       18        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       18        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4       21        0
    //  no simd        7       30        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        6       23        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        4       21        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       14        0
    //  no simd        3       20        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd4        2        7        0
    // Totals...
    // yes simd        8       22        0
    //  no simd       14       43        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        7       22        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       23        0
    //  no simd        4       32        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       23        0
    //  no simd        6       32        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for QuadNumAligningOriginAtInfinity {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        3       22        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       14        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        7        0
    fn mul(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn mul(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    fn mul_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn mul(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       15        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        4       25        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        6       27        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        6       24        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       16        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       20        0
    //  no simd        0       24        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3       20        0
    //  no simd        3       24        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       28        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       21        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7       23        0
    //  no simd        7       29        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        6       28        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       16        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        4       18        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        3       28        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        6       25        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        3       16        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       15        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       24        0
    //  no simd       15       30        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       18        0
    //  no simd        8       24        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        7       25        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       18        0
    //  no simd        4       24        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       24        0
    //  no simd        6       27        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        6       21        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for QuadNumAligningOriginAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       16        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for QuadNumAligningOriginAtInfinity {
    type Output = Infinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for QuadNumAligningOriginAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for QuadNumAligningOriginAtInfinity {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for QuadNumAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        4        0
    // no simd        4       16        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       41        0
    //    simd3        5        9        0
    // Totals...
    // yes simd       22       50        0
    //  no simd       32       68        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       18        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        1        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        3       22        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        4       21        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        6        0
    // no simd        4       24        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       14        0
    //  no simd        0       18        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for QuadNumAligningOriginAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       14        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        8        0
    fn mul(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn mul(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        8        0
    fn mul(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        8        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       20        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for QuadNumAligningOriginAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       19        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       17        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       28        0
    //  no simd       16       40        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       32        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       19        0
    //  no simd        8       25        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        7        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       20        0
    //  no simd        4       32        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       24        0
    //  no simd        8       36        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       16       32        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        7        0
    // no simd        8       28        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       17        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       20        0
    //  no simd        8       29        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for QuadNumAligningOriginAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn neg(self) -> Self {
        let negation = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ (self.group0() * Simd32x2::from(-1.0)));
        return negation;
    }
}
impl std::ops::Not for QuadNumAligningOriginAtInfinity {
    type Output = AntiQuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn not(self) -> Self::Output {
        let right_dual = AntiQuadNumAligningOriginAtInfinity::from_groups(/* e3215, scalar */ (self.group0() * Simd32x2::from(-1.0)));
        return right_dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       10        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       10        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = QuadNumAtInfinity::from_groups(/* e5, e321, e12345 */ Simd32x3::from([self.group0()[0], (other[e321] * -1.0), self.group0()[1]]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        7        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e31] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[2] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[2] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for QuadNumAligningOriginAtInfinity {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       10        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group2()[3]),
            ]),
            // e415, e425, e435, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] - other.group1()[3]), 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e415, e425, e435, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (other.group1() * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other[e3215] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([(self.group0()[0] - other[e5]), self.group0()[1]]));
        return subtraction;
    }
}
impl std::ops::SubAssign<Infinity> for QuadNumAligningOriginAtInfinity {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([(self.group0()[0] - other[e5]), self.group0()[1]]));
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[0] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        2       30        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), (-other.group0()[1] + self.group0()[1])]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (self.group0()[0] - other[e1]),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([self.group0()[1], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[1] - other[e425]), 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: MysteryQuadNum) -> Self::Output {
        let subtraction = QuadNumAtInfinity::from_groups(
            // e5, e321, e12345
            Simd32x3::from([self.group0()[0], (other.group0()[0] * -1.0), (-other.group0()[1] + self.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (self.group0()[1] - other.group0()[0]),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other[e1234] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = QuadNumAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([(other[e4] * -1.0), self.group0()[0], self.group0()[1]]));
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNum> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        2        0
    fn sub(self, other: QuadNum) -> Self::Output {
        let subtraction = QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (other.group0()[0] * -1.0),
            (self.group0()[0] - other.group0()[1]),
            (other.group0()[2] * -1.0),
            (self.group0()[1] - other.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn sub(self, other: QuadNumAligningOrigin) -> Self::Output {
        let subtraction = QuadNumAligningOrigin::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (other.group0()[0] * -1.0),
            (self.group0()[0] - other.group0()[1]),
            (self.group0()[1] - other.group0()[2]),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn sub(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ (-other.group0() + self.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    fn sub_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        let subtraction = QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ (-other.group0() + self.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn sub(self, other: QuadNumAtInfinity) -> Self::Output {
        let subtraction = QuadNumAtInfinity::from_groups(/* e5, e321, e12345 */ Simd32x3::from([
            (self.group0()[0] - other.group0()[0]),
            (other.group0()[1] * -1.0),
            (self.group0()[1] - other.group0()[2]),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: QuadNumOnOrigin) -> Self::Output {
        let subtraction = QuadNumAligningOrigin::from_groups(
            // e4, e5, e12345
            Simd32x3::from([(other.group0()[0] * -1.0), self.group0()[0], (self.group0()[1] - other.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn sub(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (other.group0()[0] * -1.0),
            (self.group0()[0] - other.group0()[1]),
            (other.group0()[2] * -1.0),
            self.group0()[1],
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] - other[e2])]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = QuadNumAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = QuadNumAligningOrigin::from_groups(
            // e4, e5, e12345
            Simd32x3::from([(other.group0()[0] * -1.0), (self.group0()[0] - other.group0()[1]), self.group0()[1]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[1] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       14        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e415, e425, e435, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (self.group0()[1] - other.group0()[0]),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        7        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[0] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e415, e425, e435, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       11        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[0] - other.group1()[3]),
            ]),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}

impl TryFrom<AntiDipoleInversion> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([anti_dipole_inversion[e5], 0.0])));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([anti_dipole_inversion_at_infinity[e5], 0.0]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([anti_dipole_inversion_orthogonal_origin[e5], 0.0]),
        ));
    }
}

impl TryFrom<AntiFlector> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([anti_flector[e5], 0.0])));
    }
}

impl TryFrom<AntiPlane> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([anti_plane[e5], 0.0])));
    }
}

impl TryFrom<CircleRotor> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, circle_rotor[e12345]])));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([0.0, circle_rotor_aligning_origin[e12345]]),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_aligning_origin_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([0.0, circle_rotor_aligning_origin_at_infinity[e12345]]),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([0.0, circle_rotor_at_infinity[e12345]]),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_rotor_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([0.0, circle_rotor_on_origin[e12345]]),
        ));
    }
}

impl TryFrom<Motor> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([motor[e5], motor[e12345]])));
    }
}

impl TryFrom<MotorAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(motor_at_infinity: MotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([motor_at_infinity[e5], 0.0])));
    }
}

impl TryFrom<MotorOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(motor_on_origin: MotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorOnOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, motor_on_origin[e12345]])));
    }
}

impl TryFrom<MultiVector> for QuadNumAligningOriginAtInfinity {
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
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
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
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([multi_vector[e5], multi_vector[e12345]]),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, mystery_circle_rotor[e12345]])));
    }
}

impl TryFrom<MysteryQuadNum> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(mystery_quad_num: MysteryQuadNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_quad_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryQuadNum do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, mystery_quad_num[e12345]])));
    }
}

impl TryFrom<MysteryVersorEven> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, mystery_versor_even[e12345]])));
    }
}

impl TryFrom<QuadNum> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(quad_num: QuadNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = quad_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = quad_num[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from QuadNum do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([quad_num[e5], quad_num[e12345]])));
    }
}

impl TryFrom<QuadNumAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(quad_num_aligning_origin: QuadNumAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = quad_num_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from QuadNumAligningOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            quad_num_aligning_origin[e5],
            quad_num_aligning_origin[e12345],
        ])));
    }
}

impl TryFrom<QuadNumAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(quad_num_at_infinity: QuadNumAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = quad_num_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from QuadNumAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([quad_num_at_infinity[e5], quad_num_at_infinity[e12345]]),
        ));
    }
}

impl TryFrom<QuadNumOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(quad_num_on_origin: QuadNumOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = quad_num_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from QuadNumOnOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, quad_num_on_origin[e12345]])));
    }
}

impl TryFrom<QuadNumOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(quad_num_orthogonal_origin: QuadNumOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = quad_num_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = quad_num_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from QuadNumOrthogonalOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([quad_num_orthogonal_origin[e5], 0.0]),
        ));
    }
}

impl TryFrom<RoundPoint> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([round_point[e5], 0.0])));
    }
}

impl TryFrom<RoundPointAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(round_point_at_origin: RoundPointAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPointAtOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([round_point_at_origin[e5], 0.0])));
    }
}

impl TryFrom<VersorEven> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([versor_even[e5], versor_even[e12345]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            versor_even_aligning_origin[e5],
            versor_even_aligning_origin[e12345],
        ])));
    }
}

impl TryFrom<VersorEvenAtInfinity> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([
            versor_even_at_infinity[e5],
            versor_even_at_infinity[e12345],
        ])));
    }
}

impl TryFrom<VersorEvenAtOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([versor_even_at_origin[e5], 0.0])));
    }
}

impl TryFrom<VersorEvenOnOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, versor_even_on_origin[e12345]])));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for QuadNumAligningOriginAtInfinity {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into QuadNumAligningOriginAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(QuadNumAligningOriginAtInfinity::from_groups(
            // e5, e12345
            Simd32x2::from([versor_even_orthogonal_origin[e5], 0.0]),
        ));
    }
}
