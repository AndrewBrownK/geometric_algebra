use crate::traits::GeometricProduct;
use crate::traits::RightDual;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 397
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:        10      13       0
//  Maximum:       170     202       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       6       0
//  Average:        11      16       0
//  Maximum:       224     256       0
impl std::ops::Add<AntiCircleOnOrigin> for Flector {
    type Output = DipoleInversion;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for Flector {
    type Output = VersorOdd;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for Flector {
    type Output = VersorOdd;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for Flector {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiMotor> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for Flector {
    type Output = VersorOddAtInfinity;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for Flector {
    type Output = VersorOdd;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<Circle> for Flector {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<DualNum> for Flector {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<FlatOrigin> for Flector {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<FlatPoint> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for Flector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for Flector {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Flector> for Flector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for Flector {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for Flector {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Horizon> for Flector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<Infinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for Flector {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for Flector {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for Flector {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for Flector {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for Flector {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for Flector {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Plane> for Flector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for Flector {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<RoundPoint> for Flector {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for Flector {
    type Output = VersorOddAtInfinity;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Add<Sphere> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<VersorEven> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for Flector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       13       20        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       13       20        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for Flector {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for Flector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for Flector {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for Flector {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for Flector {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       25       44        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for Flector {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for Flector {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for Flector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       13       20        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for Flector {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for Flector {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       13       20        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for Flector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for Flector {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for Flector {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<FlatOrigin> for Flector {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flat_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for Flector {
    fn from(from_flat_point: FlatPoint) -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ from_flat_point.group0(), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0));
    }
}

impl From<FlatPointAtInfinity> for Flector {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([from_flat_point_at_infinity[e15], from_flat_point_at_infinity[e25], from_flat_point_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlectorAtInfinity> for Flector {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([from_flector_at_infinity[e15], from_flector_at_infinity[e25], from_flector_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_flector_at_infinity[e3215]]),
        );
    }
}

impl From<FlectorOnOrigin> for Flector {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flector_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for Flector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_horizon[e3215]]),
        );
    }
}

impl From<Plane> for Flector {
    fn from(from_plane: Plane) -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ from_plane.group0());
    }
}

impl From<PlaneOnOrigin> for Flector {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_plane_on_origin[e4235], from_plane_on_origin[e4315], from_plane_on_origin[e4125], 0.0]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       69       85        0
    //  no simd       72       88        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       30       38        0
    //  no simd       36       44        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       37       49        0
    //  no simd       40       52        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       52       68        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      104      120        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       60        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       51       63        0
    //  no simd       60       72        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       64       80        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       66       82        0
    //  no simd       72       88        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       28       44        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        4       28        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       24        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for Flector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for Flector {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for Flector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for Flector {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       38       50        0
    //  no simd       44       56        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       20       48        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       64       80        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       64       80        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       32       52        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       56        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       72       88        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       36       44        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       34       48        0
    //  no simd       40       60        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       52       68        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       56        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       44        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      104      120        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       77       93        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       54       66        0
    //  no simd       60       72        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       48       64        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       66       82        0
    //  no simd       72       88        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       46        0
    //  no simd       28       52        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        4       28        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for Flector {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for Flector {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       24       32        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for Flector {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for Flector {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for Flector {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for Flector {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       32        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      142      174        0
    //    simd2        4        4        0
    //    simd3       22       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      170      202        0
    //  no simd      224      256        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       20       40        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       28       48        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       32        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       56        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       40       52        0
    //  no simd       52       64        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       56        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       46       58        0
    //  no simd       52       64        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       24        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        5        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       24       52        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Flector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       24       48        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       20       44        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       94      110        0
    //  no simd      112      128        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for Flector {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       60        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       53       65        0
    //  no simd       68       80        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       42       58        0
    //  no simd       48       64        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       64       80        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       68       84        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      100      116        0
    //  no simd      112      128        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       56       68        0
    //  no simd       68       80        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for Flector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for Flector {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<Circle> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        7        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8        3        0
    //  no simd        8       11        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        8        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8        1        0
    //  no simd        8        4        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       11        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41], other[e42], other[e43]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for Flector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        7        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<DualNum> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<FlatOrigin> for Flector {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for Flector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for Flector {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Flector> for Flector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for Flector {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for Flector {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self[e15], self[e25], self[e35], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Horizon> for Flector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<Infinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        1        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        8        9        0
    //  no simd        8       28        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for Flector {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Plane> for Flector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for Flector {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = Flector::from_groups(
            // e15, e25, e35, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<RoundPoint> for Flector {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group1(),
        );
    }
}
impl std::ops::Sub<Sphere> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for Flector {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorEven> for Flector {
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
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for Flector {
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
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       14        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        8        3        0
    //  no simd        8       12        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for Flector {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        8        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       12        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}

impl TryFrom<AntiCircleRotor> for Flector {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], anti_circle_rotor[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for Flector {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e15],
                anti_circle_rotor_aligning_origin[e25],
                anti_circle_rotor_aligning_origin[e35],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for Flector {
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
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e15],
                anti_circle_rotor_aligning_origin_at_infinity[e25],
                anti_circle_rotor_aligning_origin_at_infinity[e35],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for Flector {
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
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                anti_circle_rotor_at_infinity[e15],
                anti_circle_rotor_at_infinity[e25],
                anti_circle_rotor_at_infinity[e35],
                anti_circle_rotor_at_infinity[e45],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiLine> for Flector {
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
            let mut error = "Elements from AntiLine do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMotor> for Flector {
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
            let mut error = "Elements from AntiMotor do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for Flector {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for Flector {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        if fail {
            let mut error = "Elements from Dipole do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], dipole[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], dipole_aligning_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for Flector {
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
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], dipole_at_infinity[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for Flector {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], dipole_inversion[e45]]),
            // e4235, e4315, e4125, e3215
            dipole_inversion.group3(),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                dipole_inversion_aligning_origin[e15],
                dipole_inversion_aligning_origin[e25],
                dipole_inversion_aligning_origin[e35],
                dipole_inversion_aligning_origin[e45],
            ]),
            // e4235, e4315, e4125, e3215
            dipole_inversion_aligning_origin.group2(),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for Flector {
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
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                dipole_inversion_at_infinity[e15],
                dipole_inversion_at_infinity[e25],
                dipole_inversion_at_infinity[e35],
                dipole_inversion_at_infinity[e45],
            ]),
            // e4235, e4315, e4125, e3215
            dipole_inversion_at_infinity.group2(),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_inversion_at_origin[e15], dipole_inversion_at_origin[e25], dipole_inversion_at_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_origin[e3215]]),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion_on_origin[e4235], dipole_inversion_on_origin[e4315], dipole_inversion_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_orthogonal_origin[e3215]]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for Flector {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for Flector {
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<MysteryDipole> for Flector {
    type Error = String;
    fn try_from(mystery_dipole: MysteryDipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipole do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for Flector {
    type Error = String;
    fn try_from(mystery_dipole_inversion: MysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipoleInversion do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole_inversion[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_dipole_inversion[e4235], mystery_dipole_inversion[e4315], mystery_dipole_inversion[e4125], 0.0]),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for Flector {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        ));
    }
}

impl TryFrom<Sphere> for Flector {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            sphere.group0(),
        ));
    }
}

impl TryFrom<SphereAtOrigin> for Flector {
    type Error = String;
    fn try_from(sphere_at_origin: SphereAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereAtOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e3215]]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for Flector {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<VersorOdd> for Flector {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e45]]),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for Flector {
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
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], versor_odd_at_infinity[e45]]),
            // e4235, e4315, e4125, e3215
            versor_odd_at_infinity.group2(),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for Flector {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e15], versor_odd_orthogonal_origin[e25], versor_odd_orthogonal_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_orthogonal_origin[e3215]]),
        ));
    }
}
