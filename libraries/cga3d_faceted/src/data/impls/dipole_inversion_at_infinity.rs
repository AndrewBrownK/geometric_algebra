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
// Total Implementations: 415
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:        14      17       0
//  Maximum:       239     271       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        16      21       0
//  Maximum:       320     352       0
impl std::ops::Add<AntiCircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiDipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 1, 2, 3, 0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).extend_to_4(other[scalar]),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
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
impl std::ops::Add<AntiFlatPoint> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
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
impl std::ops::Add<AntiLine> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiPlane> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiPlaneOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiScalar> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiSphereOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<AntiVersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<Circle> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<CircleAligningOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<CircleAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<CircleAtOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<CircleOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<CircleOrthogonalOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<DualNum> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<FlatOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<FlatOrigin> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<FlatPoint> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<Flector> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Flector> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<Horizon> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Horizon> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<Infinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<Line> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<LineAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<LineOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<Motor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<MultiVector> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
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
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
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
impl std::ops::Add<MysteryCircle> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<MysteryCircleRotor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<MysteryDipole> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<MysteryDipole> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<MysteryVersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<NullDipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3(),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<Plane> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::AddAssign<Plane> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for DipoleInversionAtInfinity {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<RoundPoint> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<RoundPointAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Add<Scalar> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            self.group0(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Add<Sphere> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<VersorEven> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group1().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321]),
            // e423, e431, e412
            other.group0().truncate_to_3(),
            // e235, e315, e125
            other.group1().truncate_to_3(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45] + other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35], other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] + other[e3215]]),
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       20       32        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       17       29        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        9       20        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       23        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       14       26        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       21       32        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       14       26        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       15       26        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       11       23        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       20        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        9       20        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        9       20        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       11       23        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       10       22        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd        7       19        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       11       22        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       14       26        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       21        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       21        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for DipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for DipoleInversionAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for DipoleInversionAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for DipoleInversionAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       52        0
    //    simd3        1        4        0
    // Totals...
    // yes simd       39       56        0
    //  no simd       41       64        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       11       23        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        9       20        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for DipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for DipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       14       25        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       21       32        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for DipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       14       26        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       13        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       18       29        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       20       32        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       23        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for DipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       17       29        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiLine> for DipoleInversionAtInfinity {
    fn from(from_anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([from_anti_line[e23], from_anti_line[e31], from_anti_line[e12], 0.0]),
            // e15, e25, e35
            from_anti_line.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([from_anti_line_on_origin[e23], from_anti_line_on_origin[e31], from_anti_line_on_origin[e12], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtInfinity> for DipoleInversionAtInfinity {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            from_dipole_at_infinity.group0(),
            // e15, e25, e35
            from_dipole_at_infinity.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatOrigin> for DipoleInversionAtInfinity {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(from_flat_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for DipoleInversionAtInfinity {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(from_flat_point[e45]),
            // e15, e25, e35
            from_flat_point.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flat_point_at_infinity.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for DipoleInversionAtInfinity {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(from_flector[e45]),
            // e15, e25, e35
            from_flector.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            from_flector.group1(),
        );
    }
}

impl From<FlectorAtInfinity> for DipoleInversionAtInfinity {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flector_at_infinity.group0().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(from_flector_at_infinity[e3215]),
        );
    }
}

impl From<FlectorOnOrigin> for DipoleInversionAtInfinity {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(from_flector_on_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for DipoleInversionAtInfinity {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(from_horizon[e3215]),
        );
    }
}

impl From<MysteryDipole> for DipoleInversionAtInfinity {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            from_mystery_dipole.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            from_mystery_dipole_inversion.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_mystery_dipole_inversion[e4235], from_mystery_dipole_inversion[e4315], from_mystery_dipole_inversion[e4125], 0.0]),
        );
    }
}

impl From<Plane> for DipoleInversionAtInfinity {
    fn from(from_plane: Plane) -> Self {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            from_plane.group0(),
        );
    }
}

impl From<PlaneOnOrigin> for DipoleInversionAtInfinity {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([from_plane_on_origin[e4235], from_plane_on_origin[e4315], from_plane_on_origin[e4125], 0.0]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       62       78        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      101        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       90      106        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74       90        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       79       95        0
    //  no simd       94      110        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       57        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       47       59        0
    //  no simd       53       65        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       55       67        0
    //  no simd       64       76        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       69        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       73       89        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      121      137        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      128      144        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       81       93        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       84       96        0
    //  no simd       93      105        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       70       86        0
    //  no simd       88      104        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       81       97        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       87      103        0
    //  no simd      105      121        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       40       57        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        2        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       13        0
    //  no simd       10       30        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       19        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       24        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       21       40        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       54       66        0
    //  no simd       60       72        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       32       44        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       42       54        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       60        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       51       63        0
    //  no simd       60       72        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       32       44        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       47        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       37       49        0
    //  no simd       43       55        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       69        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       59       71        0
    //  no simd       65       77        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       45        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       27       47        0
    //  no simd       32       52        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       67       83        0
    //  no simd       88      104        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78       94        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       82       98        0
    //  no simd       94      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       87        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       83       99        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       57        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       47       59        0
    //  no simd       53       65        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       54        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       51       66        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       62       78        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       62        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       49       66        0
    //  no simd       61       78        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      101        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       90      106        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78       94        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       82       98        0
    //  no simd       94      110        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       61        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       50       62        0
    //  no simd       53       65        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       55       67        0
    //  no simd       64       76        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       73       89        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82       98        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       85      101        0
    //  no simd       94      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       65        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       52       68        0
    //  no simd       61       77        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       61        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       50       62        0
    //  no simd       53       65        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       54        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       42       57        0
    //  no simd       51       66        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      125      141        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      131      147        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85       97        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       87       99        0
    //  no simd       93      105        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       68        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       58       73        0
    //  no simd       73       88        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       73       89        0
    //  no simd       88      104        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       81       97        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       87      103        0
    //  no simd      105      121        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       39        0
    //    simd3        2        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       27       45        0
    //  no simd       40       61        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       87        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       83       99        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       11        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       10       27        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       23        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       21       36        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       17       21        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       68        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       57       69        0
    //  no simd       60       72        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       25        0
    //  no simd       24       28        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       44        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for DipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for DipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        7        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       42       54        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for DipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       17       21        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       54       66        0
    //  no simd       60       72        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       25        0
    //  no simd       24       28        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       32       44        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      198      230        0
    //    simd2        7        7        0
    //    simd3       28       28        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      239      271        0
    //  no simd      320      352        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       32       44        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       34       46        0
    //  no simd       43       55        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       44        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       65       77        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       72        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       64       76        0
    //  no simd       76       88        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       70       82        0
    //  no simd       76       88        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       34       45        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       30        0
    //    simd3        1        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       34       45        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       34        0
    //    simd3        2        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       30       41        0
    //  no simd       49       60        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        3       20        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       40        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       49       60        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        3       23        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       48        0
    //    simd3        1        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       31       51        0
    //  no simd       39       59        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        2        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       10        0
    //  no simd        6       30        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for DipoleInversionAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       42        0
    //    simd3        3        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       27       47        0
    //  no simd       39       59        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        6       23        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       41        0
    //    simd3        1        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       24       44        0
    //  no simd       32       52        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      140        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      133      149        0
    //  no simd      160      176        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       95      111        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84       96        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       89      101        0
    //  no simd      104      116        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       64        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       55       70        0
    //  no simd       73       88        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       70       86        0
    //  no simd       88      104        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       95      111        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      128      144        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      136      152        0
    //  no simd      160      176        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      100        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       92      104        0
    //  no simd      104      116        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      100        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      116      132        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn neg(self) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for DipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for DipoleInversionAtInfinity {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).extend_to_4(other[scalar] * -1.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
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
impl std::ops::Sub<AntiFlatPoint> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
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
impl std::ops::Sub<AntiLine> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiPlane> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiPlaneOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiScalar> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiSphereOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<AntiVersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<Circle> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<CircleAligningOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<CircleAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<CircleAtOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<CircleOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<CircleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        3        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        2        0
    //  no simd       11        7        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        7        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        7        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        4        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        7        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        3        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<DualNum> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<FlatOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlatOrigin> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<FlatPoint> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<Flector> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Flector> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<Horizon> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Horizon> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<Infinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<Line> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<LineAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<LineOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<Motor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<MultiVector> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       11        8        0
    //  no simd       11       25        0
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
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
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
impl std::ops::Sub<MysteryCircle> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<MysteryCircleRotor> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<MysteryDipole> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<MysteryDipole> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<MysteryVersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<NullDipoleAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<Plane> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::SubAssign<Plane> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for DipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<RoundPoint> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<RoundPointAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
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
impl std::ops::Sub<Scalar> for DipoleInversionAtInfinity {
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
            self.group0(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
    }
}
impl std::ops::Sub<Sphere> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        1        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for DipoleInversionAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0(),
            // e15, e25, e35, e1234
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorEven> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       14        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for DipoleInversionAtInfinity {
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
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for DipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for DipoleInversionAtInfinity {
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
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for DipoleInversionAtInfinity {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x3::from(0.0).extend_to_4(self[e45]),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0().truncate_to_3(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).extend_to_4(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11        2        0
    //  no simd       11        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for DipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11        1        0
    //  no simd       11        4        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45] - other[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125], self[e3215] - other[e3215]]),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for DipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        8        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], other[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215] - other[e3215]]),
        );
    }
}

impl TryFrom<AntiCircleOnOrigin> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotor> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
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
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            anti_circle_rotor.group1(),
            // e15, e25, e35
            anti_circle_rotor.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for DipoleInversionAtInfinity {
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
        let el = anti_circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e23],
                anti_circle_rotor_aligning_origin[e31],
                anti_circle_rotor_aligning_origin[e12],
                0.0,
            ]),
            // e15, e25, e35
            anti_circle_rotor_aligning_origin.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e23],
                anti_circle_rotor_aligning_origin_at_infinity[e31],
                anti_circle_rotor_aligning_origin_at_infinity[e12],
                0.0,
            ]),
            // e15, e25, e35
            anti_circle_rotor_aligning_origin_at_infinity.group1().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            anti_circle_rotor_at_infinity.group0(),
            // e15, e25, e35
            anti_circle_rotor_at_infinity.group1().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor_on_origin[e23], anti_circle_rotor_on_origin[e31], anti_circle_rotor_on_origin[e12], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMotor> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
            // e15, e25, e35
            anti_motor.group1().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(anti_motor[e3215]),
        ));
    }
}

impl TryFrom<AntiMotorOnOrigin> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotorOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            anti_mystery_circle_rotor.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([anti_versor_even_on_origin[e23], anti_versor_even_on_origin[e31], anti_versor_even_on_origin[e12], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
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
        if fail {
            let mut error = "Elements from Dipole do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            dipole.group1(),
            // e15, e25, e35
            dipole.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from DipoleAligningOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(dipole_aligning_origin[e45]),
            // e15, e25, e35
            dipole_aligning_origin.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from DipoleAtOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            dipole_at_origin.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
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
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            dipole_inversion.group1(),
            // e15, e25, e35
            dipole_inversion.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            dipole_inversion.group3(),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(dipole_inversion_aligning_origin[e45]),
            // e15, e25, e35
            dipole_inversion_aligning_origin.group1().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            dipole_inversion_aligning_origin.group2(),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            dipole_inversion_at_origin.group1().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(dipole_inversion_at_origin[e3215]),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(dipole_inversion_on_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion_on_origin[e4235], dipole_inversion_on_origin[e4315], dipole_inversion_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for DipoleInversionAtInfinity {
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
        let el = dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e23],
                dipole_inversion_orthogonal_origin[e31],
                dipole_inversion_orthogonal_origin[e12],
                0.0,
            ]),
            // e15, e25, e35
            dipole_inversion_orthogonal_origin.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(dipole_inversion_orthogonal_origin[e3215]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from DipoleOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x3::from(0.0).extend_to_4(dipole_on_origin[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for DipoleInversionAtInfinity {
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
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12], 0.0]),
            // e15, e25, e35
            dipole_orthogonal_origin.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from MultiVector do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35
            multi_vector.group4(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for DipoleInversionAtInfinity {
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
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            mystery_versor_odd.group1(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        ));
    }
}

impl TryFrom<Sphere> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from Sphere do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            sphere.group0(),
        ));
    }
}

impl TryFrom<SphereAtOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from SphereAtOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(sphere_at_origin[e3215]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for DipoleInversionAtInfinity {
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
            let mut error = "Elements from SphereOnOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<VersorOdd> for DipoleInversionAtInfinity {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
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
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            versor_odd.group1(),
            // e15, e25, e35
            versor_odd.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            versor_odd.group3(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for DipoleInversionAtInfinity {
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
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            versor_odd_at_infinity.group1(),
            // e15, e25, e35
            Simd32x3::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35]]),
            // e4235, e4315, e4125, e3215
            versor_odd_at_infinity.group2(),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for DipoleInversionAtInfinity {
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
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into DipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e23], versor_odd_orthogonal_origin[e31], versor_odd_orthogonal_origin[e12], 0.0]),
            // e15, e25, e35
            versor_odd_orthogonal_origin.group2().truncate_to_3(),
            // e4235, e4315, e4125, e3215
            Simd32x3::from(0.0).extend_to_4(versor_odd_orthogonal_origin[e3215]),
        ));
    }
}
