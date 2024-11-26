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
// Total Implementations: 377
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       2       0
//  Average:         9      13       0
//  Maximum:       158     190       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       5       0
//  Average:        10      15       0
//  Maximum:       192     224       0
impl std::ops::Add<AntiCircleOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(other[e5]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e4]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Add<AntiDualNum> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<AntiFlector> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiLine> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
        );
    }
}
impl std::ops::Add<AntiPlane> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
        );
    }
}
impl std::ops::Add<AntiScalar> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<CircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], other[e12345] + self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
        );
    }
}
impl std::ops::Add<Dipole> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(other[e45]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other[e5]]),
        );
    }
}
impl std::ops::Add<Line> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e12345]),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
        );
    }
}
impl std::ops::Add<Motor> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], 0.0]),
            // e235, e315, e125, e5
            other.group1(),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            other.group0(),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
        );
    }
}
impl std::ops::Add<MultiVector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for CircleRotorOnOrigin {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
        );
    }
}
impl std::ops::Add<Origin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
        );
    }
}
impl std::ops::Add<Plane> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for CircleRotorOnOrigin {
    type Output = VersorEven;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other[e5]]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other[e5]]),
        );
    }
}
impl std::ops::Add<Scalar> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e4]]),
            // e235, e315, e125, e5
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            other.group1(),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e4]]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       14       29        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for CircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for CircleRotorOnOrigin {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_scalar[e12345]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
    }
}

impl From<CircleOnOrigin> for CircleRotorOnOrigin {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_circle_on_origin[e423], from_circle_on_origin[e431], from_circle_on_origin[e412], 0.0]),
            // e415, e425, e435
            from_circle_on_origin.group1(),
        );
    }
}

impl From<LineOnOrigin> for CircleRotorOnOrigin {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return CircleRotorOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(0.0), /* e415, e425, e435 */ from_line_on_origin.group0());
    }
}

impl From<MotorOnOrigin> for CircleRotorOnOrigin {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_motor_on_origin[e12345]]),
            // e415, e425, e435
            Simd32x3::from([from_motor_on_origin[e415], from_motor_on_origin[e425], from_motor_on_origin[e435]]),
        );
    }
}

impl From<NullCircleAtOrigin> for CircleRotorOnOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([from_null_circle_at_origin[e423], from_null_circle_at_origin[e431], from_null_circle_at_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       33        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       61       77        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       42       58        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       46       62        0
    //  no simd       49       65        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       32       40        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73       89        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       77       93        0
    //  no simd       89      105        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       77        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       64       80        0
    //  no simd       73       89        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       41        0
    //  no simd       36       44        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       61       77        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       19        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       11        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       22       37        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       52       68        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       35       51        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       46       62        0
    //  no simd       52       68        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       20       34        0
    //  no simd       23       43        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       37       49        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for CircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       16       43        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       14        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       17       25        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       30       38        0
    //  no simd       36       44        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       47       63        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       42       58        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       26       42        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       33        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       33       49        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       61       77        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       42       58        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       46       62        0
    //  no simd       49       65        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       32       40        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       33       49        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       42       58        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       26       42        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      101        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       86      102        0
    //  no simd       89      105        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       68       84        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       69       85        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       70       86        0
    //  no simd       73       89        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       37       53        0
    //  no simd       40       56        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       36       44        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       61       77        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       11       27        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       47       63        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        3       19        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       22       37        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       52       68        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       28        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       14        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for CircleRotorOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       21        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       35       51        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       52       68        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      143      175        0
    //    simd3       11       11        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      158      190        0
    //  no simd      192      224        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       16       32        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       23       39        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       16       30        0
    //  no simd       16       36        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       37       49        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       41       53        0
    //  no simd       44       56        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       41       53        0
    //  no simd       44       56        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for CircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for CircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       13        0
    //  no simd       12       16        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for CircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       31        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for CircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd3        1        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       18       36        0
    //  no simd       20       47        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for CircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       29        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for CircleRotorOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       20       35        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       18        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       17       25        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       81       97        0
    //  no simd       96      112        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       68       84        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       37       53        0
    //  no simd       40       56        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       30       38        0
    //  no simd       36       44        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       68       84        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       90      106        0
    //  no simd       96      112        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       80       96        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn neg(self) -> Self::Output {
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0() * Simd32x4::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Not for CircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6       12        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3       12        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        8        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235], other[e315], other[e125], other[e5]]) * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiLine> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, 1.0, other[e5]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        8        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        8        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345] - other[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345] - other[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345] - other[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345] - other[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::Sub<Dipole> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for CircleRotorOnOrigin {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for CircleRotorOnOrigin {
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            other.group1() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0() * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            other.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group1() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, 1.0, other[e5]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Line> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for CircleRotorOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235], other[e315], other[e125], self[e12345]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::Sub<Motor> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], 0.0]),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
        );
    }
}
impl std::ops::Sub<MultiVector> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        7       10        0
    //  no simd        7       29        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345] - other[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            other.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            other.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345]]),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for CircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for CircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for CircleRotorOnOrigin {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
        );
    }
}
impl std::ops::Sub<Origin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
        );
    }
}
impl std::ops::Sub<Plane> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, 1.0, other[e5]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([1.0, 1.0, 1.0, other[e5]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Scalar> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for CircleRotorOnOrigin {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for CircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        8        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        4       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423], self[e431], self[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            other.group2() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for CircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            other.group1() * Simd32x4::from(-1.0),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOdd> for CircleRotorOnOrigin {
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
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], other[e45]]) * Simd32x4::from(-1.0),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for CircleRotorOnOrigin {
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
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([1.0, 1.0, 1.0, other[e45]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for CircleRotorOnOrigin {
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
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self[e12345]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other[e41], other[e42], other[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e15, e25, e35
            Simd32x3::from([other[e15], other[e25], other[e35]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([other[e23], other[e31], other[e12]]) * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_inversion[e423], anti_dipole_inversion[e431], anti_dipole_inversion[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = anti_dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
            ]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e423],
                anti_dipole_inversion_orthogonal_origin[e431],
                anti_dipole_inversion_orthogonal_origin[e412],
                0.0,
            ]),
            // e415, e425, e435
            anti_dipole_inversion_orthogonal_origin.group1(),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from([anti_mystery_dipole_inversion[e415], anti_mystery_dipole_inversion[e425], anti_mystery_dipole_inversion[e435]]),
        ));
    }
}

impl TryFrom<Circle> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle[e423], circle[e431], circle[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from([circle[e415], circle[e425], circle[e435]]),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412], 0.0]),
            // e415, e425, e435
            circle_aligning_origin.group1(),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435]]),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from CircleRotor do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412], circle_rotor[e12345]]),
            // e415, e425, e435
            Simd32x3::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435]]),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                circle_rotor_aligning_origin[e423],
                circle_rotor_aligning_origin[e431],
                circle_rotor_aligning_origin[e412],
                circle_rotor_aligning_origin[e12345],
            ]),
            // e415, e425, e435
            circle_rotor_aligning_origin.group1(),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, circle_rotor_aligning_origin_at_infinity[e12345]]),
            // e415, e425, e435
            circle_rotor_aligning_origin_at_infinity.group0(),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from CircleRotorAtInfinity do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, circle_rotor_at_infinity[e12345]]),
            // e415, e425, e435
            Simd32x3::from([circle_rotor_at_infinity[e415], circle_rotor_at_infinity[e425], circle_rotor_at_infinity[e435]]),
        ));
    }
}

impl TryFrom<DualNum> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<Line> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435
            line.group0(),
        ));
    }
}

impl TryFrom<Motor> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor[e12345]]),
            // e415, e425, e435
            Simd32x3::from([motor[e415], motor[e425], motor[e435]]),
        ));
    }
}

impl TryFrom<MultiVector> for CircleRotorOnOrigin {
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
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from MultiVector do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e12345]]),
            // e415, e425, e435
            Simd32x3::from([multi_vector[e415], multi_vector[e425], multi_vector[e435]]),
        ));
    }
}

impl TryFrom<MysteryCircle> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(mystery_circle: MysteryCircle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircle do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435]]),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_circle_rotor[e12345]]),
            // e415, e425, e435
            Simd32x3::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435]]),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for CircleRotorOnOrigin {
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
        let el = mystery_versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_even[e12345]]),
            // e415, e425, e435
            Simd32x3::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435]]),
        ));
    }
}

impl TryFrom<NullVersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullVersorEvenAtOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<VersorEven> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
            let mut error = "Elements from VersorEven do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            versor_even.group0(),
            // e415, e425, e435
            Simd32x3::from([versor_even[e415], versor_even[e425], versor_even[e435]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = versor_even_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            versor_even_aligning_origin.group0(),
            // e415, e425, e435
            Simd32x3::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435]]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for CircleRotorOnOrigin {
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
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_infinity[e12345]]),
            // e415, e425, e435
            Simd32x3::from([versor_even_at_infinity[e415], versor_even_at_infinity[e425], versor_even_at_infinity[e435]]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = versor_even_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            versor_even_on_origin.group0(),
            // e415, e425, e435
            Simd32x3::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435]]),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for CircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = versor_even_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into CircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435
            Simd32x3::from(0.0),
        ));
    }
}
