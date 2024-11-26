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
// Total Implementations: 390
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         4       8       0
//  Maximum:        68      95       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       6       0
//  Average:         5      11       0
//  Maximum:        96     129       0
impl std::ops::Add<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, scalar
            other.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, scalar
            other.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, scalar
            other.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, scalar
            other.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDualNum> for DipoleOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlector> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLine> for DipoleOnOrigin {
    type Output = Dipole;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            other.group1(),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for DipoleOnOrigin {
    type Output = Dipole;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Add<AntiMotor> for DipoleOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
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
    }
}
impl std::ops::Add<AntiScalar> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
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
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<Circle> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Dipole> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            other.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35
            other.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            other.group1(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], other[e45] + self[e45]]),
            // e15, e25, e35, e1234
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35, e1234
            other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e1234, e4235, e4315, e4125
            other.group1(),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
        );
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for DipoleOnOrigin {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43]]),
            // e23, e31, e12, e45
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e45]),
            // e15, e25, e35
            other.group2(),
        );
    }
}
impl std::ops::Add<DualNum> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<FlatOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]));
    }
}
impl std::ops::AddAssign<FlatOrigin> for DipoleOnOrigin {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]));
    }
}
impl std::ops::Add<FlatPoint> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        return DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0(), /* e15, e25, e35 */ other.group0());
    }
}
impl std::ops::Add<Flector> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
        );
    }
}
impl std::ops::Add<Horizon> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<Infinity> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<Line> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Motor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
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
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45] + other[e45]]),
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
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipole> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for DipoleOnOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Origin> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<Plane> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: Plane) -> Self::Output {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
        );
    }
}
impl std::ops::Add<RoundPoint> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<RoundPointAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Add<Scalar> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other[scalar]]),
        );
    }
}
impl std::ops::Add<Sphere> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e1234]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
        );
    }
}
impl std::ops::Add<VersorEven> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] + other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       17        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for DipoleOnOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       17        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       17        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for DipoleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for DipoleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for DipoleOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for DipoleOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for DipoleOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       17       32        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for DipoleOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for DipoleOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       17        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for DipoleOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for DipoleOnOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for DipoleOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for DipoleOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        9       16        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<FlatOrigin> for DipoleOnOrigin {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, from_flat_origin[e45]]));
    }
}

impl From<NullDipoleAtOrigin> for DipoleOnOrigin {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([from_null_dipole_at_origin[e41], from_null_dipole_at_origin[e42], from_null_dipole_at_origin[e43], 0.0]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       41        0
    //  no simd       29       44        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       40        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       22       37        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       26       41        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       19        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       45       60        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       40       57        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for DipoleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       12       28        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       28       44        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for DipoleOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       11       33        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       28       44        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for DipoleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       44        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       16        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       11       28        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       17       40        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for DipoleOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       19        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for DipoleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       25       44        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       21       40        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       22       41        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       10       32        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       19        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       27        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       14       35        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       29       48        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       25       44        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       22       41        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       45        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       11       27        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       37        0
    //  no simd       25       40        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       27        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       14       35        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       22       37        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       10       32        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       44       60        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       37       53        0
    //  no simd       40       56        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       16       31        0
    //  no simd       16       40        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for DipoleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       44        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       36        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       11       29        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       21        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       28       44        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       28        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for DipoleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       16        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for DipoleOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for DipoleOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       18       37        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for DipoleOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       10       25        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       28       41        0
    //  no simd       28       44        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for DipoleOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       16       28        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       78        0
    //    simd2        3        3        0
    //    simd3        8       11        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       68       95        0
    //  no simd       96      129        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       11       32        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       20        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       17       36        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       20       32        0
    //  no simd       20       44        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       20       36        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for DipoleOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for DipoleOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for DipoleOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for DipoleOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for DipoleOnOrigin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       23        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for DipoleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for DipoleOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       20        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for DipoleOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd        9       24        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       20        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for DipoleOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       13        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       31       43        0
    //  no simd       49       64        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       32       48        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       32       46        0
    //  no simd       44       61        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       16       31        0
    //  no simd       16       40        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for DipoleOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       12       28        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       32       48        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       42       58        0
    //  no simd       48       64        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       44       60        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn neg(self) -> Self::Output {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0() * Simd32x4::from(-1.0));
    }
}
impl std::ops::Not for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDualNum> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlector> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLine> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Sub<AntiMotor> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<AntiScalar> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
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
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<Circle> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Dipole> for DipoleOnOrigin {
    type Output = Dipole;
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
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       12        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            other.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        1       12        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35, e1234
            other.group1() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
        );
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for DipoleOnOrigin {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for DipoleOnOrigin {
    type Output = Dipole;
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
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<DualNum> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<FlatOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]));
    }
}
impl std::ops::SubAssign<FlatOrigin> for DipoleOnOrigin {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]));
    }
}
impl std::ops::Sub<FlatPoint> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        3        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for DipoleOnOrigin {
    type Output = DipoleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        return DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ self.group0(), /* e15, e25, e35 */ other.group0() * Simd32x3::from(-1.0));
    }
}
impl std::ops::Sub<Flector> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Horizon> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Infinity> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<Line> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Motor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MultiVector> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       28        0
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
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipole> for DipoleOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for DipoleOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        1        8        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        1       12        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for DipoleOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for DipoleOnOrigin {
    type Output = DipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for DipoleOnOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Origin> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<Plane> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<RoundPoint> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
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
    }
}
impl std::ops::Sub<Scalar> for DipoleOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self[e41], self[e42], self[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Sphere> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAligningOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for DipoleOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOnOrigin::from_groups(
            // e41, e42, e43, e45
            self.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEven> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for DipoleOnOrigin {
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
            self.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorOdd> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        4       16        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1       16        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45] - other[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for DipoleOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       16        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], self[e45]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e15, e25, e35, e1234
            other.group2() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([1.0, 1.0, 1.0, other[e3215]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}

impl TryFrom<AntiCircleOnOrigin> for DipoleOnOrigin {
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
            let mut error = "Elements from AntiCircleOnOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            anti_circle_on_origin[e41],
            anti_circle_on_origin[e42],
            anti_circle_on_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<AntiCircleRotor> for DipoleOnOrigin {
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
        let el = anti_circle_rotor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotor do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            anti_circle_rotor[e41],
            anti_circle_rotor[e42],
            anti_circle_rotor[e43],
            anti_circle_rotor[e45],
        ])));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for DipoleOnOrigin {
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
        let el = anti_circle_rotor_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            anti_circle_rotor_aligning_origin[e41],
            anti_circle_rotor_aligning_origin[e42],
            anti_circle_rotor_aligning_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for DipoleOnOrigin {
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
        let el = anti_circle_rotor_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_at_infinity[e45]]),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for DipoleOnOrigin {
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
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            anti_circle_rotor_on_origin[e41],
            anti_circle_rotor_on_origin[e42],
            anti_circle_rotor_on_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for DipoleOnOrigin {
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
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[e45]]),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for DipoleOnOrigin {
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
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            anti_versor_even_on_origin[e41],
            anti_versor_even_on_origin[e42],
            anti_versor_even_on_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<Dipole> for DipoleOnOrigin {
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
        let el = dipole[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], dipole[e45]]),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ dipole_aligning_origin.group0()));
    }
}

impl TryFrom<DipoleAtInfinity> for DipoleOnOrigin {
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
        let el = dipole_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, dipole_at_infinity[e45]])));
    }
}

impl TryFrom<DipoleAtOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversion> for DipoleOnOrigin {
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
        let el = dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            dipole_inversion[e41],
            dipole_inversion[e42],
            dipole_inversion[e43],
            dipole_inversion[e45],
        ])));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ dipole_inversion_aligning_origin.group0()));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for DipoleOnOrigin {
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
        let el = dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_infinity[e45]]),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            dipole_inversion_at_origin[e41],
            dipole_inversion_at_origin[e42],
            dipole_inversion_at_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ dipole_inversion_on_origin.group0()));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
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
        let el = dipole_inversion_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            dipole_inversion_orthogonal_origin[e41],
            dipole_inversion_orthogonal_origin[e42],
            dipole_inversion_orthogonal_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for DipoleOnOrigin {
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
        let el = dipole_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            dipole_orthogonal_origin[e41],
            dipole_orthogonal_origin[e42],
            dipole_orthogonal_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<FlatPoint> for DipoleOnOrigin {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flat_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flat_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]])));
    }
}

impl TryFrom<Flector> for DipoleOnOrigin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, flector[e45]])));
    }
}

impl TryFrom<FlectorOnOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(flector_on_origin: FlectorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlectorOnOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]])));
    }
}

impl TryFrom<MultiVector> for DipoleOnOrigin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
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
            let mut error = "Elements from MultiVector do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ multi_vector.group3()));
    }
}

impl TryFrom<MysteryDipole> for DipoleOnOrigin {
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
            let mut error = "Elements from MysteryDipole do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole[e45]])));
    }
}

impl TryFrom<MysteryDipoleInversion> for DipoleOnOrigin {
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
        let el = mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipoleInversion do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole_inversion[e45]]),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for DipoleOnOrigin {
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
        let el = mystery_versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
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
            let mut error = "Elements from MysteryVersorOdd do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[e45]])));
    }
}

impl TryFrom<NullDipoleInversionAtOrigin> for DipoleOnOrigin {
    type Error = String;
    fn try_from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullDipoleInversionAtOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            null_dipole_inversion_at_origin[e41],
            null_dipole_inversion_at_origin[e42],
            null_dipole_inversion_at_origin[e43],
            0.0,
        ])));
    }
}

impl TryFrom<VersorOdd> for DipoleOnOrigin {
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
        let el = versor_odd[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = versor_odd[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[e45]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for DipoleOnOrigin {
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
        let el = versor_odd_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = versor_odd_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[e45]])));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for DipoleOnOrigin {
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
        let el = versor_odd_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into DipoleOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from([
            versor_odd_orthogonal_origin[e41],
            versor_odd_orthogonal_origin[e42],
            versor_odd_orthogonal_origin[e43],
            0.0,
        ])));
    }
}
