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
// Total Implementations: 389
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:        14      17       0
//  Maximum:       233     265       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       5       0
//  Average:        14      20       0
//  Maximum:       288     320       0
impl std::ops::Add<AntiCircleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for CircleRotorAligningOrigin {
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
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412], self[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e4]]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<AntiFlector> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiLine> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
        );
    }
}
impl std::ops::Add<AntiPlane> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
        );
    }
}
impl std::ops::Add<AntiScalar> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotor> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e415, e425, e435
            Simd32x3::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125], other[e12345] + self[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<Dipole> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] + other[e12345])),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<Line> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<Line> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345]]),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<Motor> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] + other[e12345])),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<MultiVector> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
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
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] + other[e12345]]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] + other[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<Origin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<Plane> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<Scalar> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e4]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] + other[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e4]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345] + other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], other[e4]]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(other[e321]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125], other[e5]]),
            // e1, e2, e3, e4
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       19        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       19        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       13        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       13        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       22       41        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       13        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       19        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       19        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for CircleRotorAligningOrigin {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for CircleRotorAligningOrigin {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_scalar[e12345]]),
        );
    }
}

impl From<CircleAligningOrigin> for CircleRotorAligningOrigin {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e415, e425, e435
            from_circle_aligning_origin.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([from_circle_aligning_origin[e235], from_circle_aligning_origin[e315], from_circle_aligning_origin[e125], 0.0]),
        );
    }
}

impl From<CircleAtOrigin> for CircleRotorAligningOrigin {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([from_circle_at_origin[e235], from_circle_at_origin[e315], from_circle_at_origin[e125], 0.0]),
        );
    }
}

impl From<CircleOnOrigin> for CircleRotorAligningOrigin {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e415, e425, e435
            from_circle_on_origin.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            from_circle_rotor_aligning_origin_at_infinity.group0(),
            // e235, e315, e125, e12345
            from_circle_rotor_aligning_origin_at_infinity.group1(),
        );
    }
}

impl From<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([from_circle_rotor_on_origin[e423], from_circle_rotor_on_origin[e431], from_circle_rotor_on_origin[e412]]),
            // e415, e425, e435
            from_circle_rotor_on_origin.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_circle_rotor_on_origin[e12345]]),
        );
    }
}

impl From<Line> for CircleRotorAligningOrigin {
    fn from(from_line: Line) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            from_line.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from([from_line[e235], from_line[e315], from_line[e125], 0.0]),
        );
    }
}

impl From<LineAtInfinity> for CircleRotorAligningOrigin {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([from_line_at_infinity[e235], from_line_at_infinity[e315], from_line_at_infinity[e125], 0.0]),
        );
    }
}

impl From<LineOnOrigin> for CircleRotorAligningOrigin {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            from_line_on_origin.group0(),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorOnOrigin> for CircleRotorAligningOrigin {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([from_motor_on_origin[e415], from_motor_on_origin[e425], from_motor_on_origin[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, from_motor_on_origin[e12345]]),
        );
    }
}

impl From<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       80       96        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       81       97        0
    //  no simd       84      100        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      118      134        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      122      138        0
    //  no simd      134      150        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82       98        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       85      101        0
    //  no simd       94      110        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       25       44        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        6       28        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for CircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       18        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       40        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for CircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       64       80        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       46        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       31       49        0
    //  no simd       34       58        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for CircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       25       44        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for CircleRotorAligningOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       20        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for CircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       25       44        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       84      100        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       74       90        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       80       96        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       81       97        0
    //  no simd       84      100        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       54       70        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       84      100        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       54       70        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      118      134        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      122      138        0
    //  no simd      134      150        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      104      120        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78       94        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       82       98        0
    //  no simd       94      110        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       64       80        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      106        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       91      107        0
    //  no simd       94      110        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       41        0
    //  no simd       25       44        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       74       90        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       23        0
    //  no simd        6       32        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       22        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       40        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for CircleRotorAligningOrigin {
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
impl std::ops::Mul<FlectorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for CircleRotorAligningOrigin {
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
impl std::ops::Mul<Infinity> for CircleRotorAligningOrigin {
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
impl std::ops::Mul<Line> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       60        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       28       40        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      208      240        0
    //    simd3       20       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      233      265        0
    //  no simd      288      320        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       40        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       46        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       34       50        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       24       48        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       58        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       54       70        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       46       59        0
    //  no simd       64       80        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       22        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       40        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for CircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       25        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for CircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       37        0
    //  no simd       25       40        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for CircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       35       50        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for CircleRotorAligningOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       21        0
    //  no simd        6       31        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for CircleRotorAligningOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for CircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        1        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       45        0
    //  no simd       35       50        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for CircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        6       20        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for CircleRotorAligningOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       25       40        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      140        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      129      145        0
    //  no simd      144      160        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      104      120        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      104        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      104      120        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       64       80        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      104      120        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      112      128        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      120      136        0
    //  no simd      144      160        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       80       96        0
    //  no simd      104      120        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      104      120        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn neg(self) -> Self::Output {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0() * Simd32x3::from(-1.0),
            // e415, e425, e435
            self.group1() * Simd32x3::from(-1.0),
            // e235, e315, e125, e12345
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for CircleRotorAligningOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        9        3        0
    //  no simd        9       12        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6       12        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for CircleRotorAligningOrigin {
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
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9        2        0
    //  no simd        9        8        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for CircleRotorAligningOrigin {
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
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiLine> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for CircleRotorAligningOrigin {
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
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9        1        0
    //  no simd        9        4        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        1        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleRotor> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        1        0
    //  no simd       10        4        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<Dipole> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            other.group1() * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] - other[e12345])),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<Infinity> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Line> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::SubAssign<Line> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], self[e12345]]),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<Motor> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] - other[e12345])),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435
            Simd32x3::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<MultiVector> for CircleRotorAligningOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       10        9        0
    //  no simd       10       26        0
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
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([self[e235], self[e315], self[e125], self[e12345] - other[e12345]]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for CircleRotorAligningOrigin {
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
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] - other[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for CircleRotorAligningOrigin {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e415, e425, e435
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<Origin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<Plane> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for CircleRotorAligningOrigin {
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
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4(self[e12345]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<Scalar> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       10        3        0
    //  no simd       10       12        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10        2        0
    //  no simd       10        8        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        7        3        0
    //  no simd        7       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            crate::swizzle!(self.group0(), 0, 1, 2).extend_to_4((self[e12345] - other[e12345])),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e321]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        5        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e4
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e4] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345] - other[e12345]]),
            // e415, e425, e435, e4
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], other[e4]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235], self[e315], self[e125], 0.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for CircleRotorAligningOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6        9        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412], self[e12345]]),
            // e415, e425, e435, e321
            crate::swizzle!(self.group1(), 0, 1, 2).extend_to_4((other[e321] * -1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125], other[e5]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOdd> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([1.0, other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([0.0, -1.0, -1.0, -1.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self[e235], self[e315], self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 1.0, 1.0, 1.0]) * Simd32x4::from([-1.0, 0.0, 0.0, 0.0]),
            // e3215
            other[e3215] * -1.0,
        );
    }
}

impl TryFrom<AntiDipoleInversion> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiDipoleInversion do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            anti_dipole_inversion.group0(),
            // e415, e425, e435
            Simd32x3::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for CircleRotorAligningOrigin {
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
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([
                anti_dipole_inversion_orthogonal_origin[e423],
                anti_dipole_inversion_orthogonal_origin[e431],
                anti_dipole_inversion_orthogonal_origin[e412],
            ]),
            // e415, e425, e435
            anti_dipole_inversion_orthogonal_origin.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiFlatPoint> for CircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiFlector> for CircleRotorAligningOrigin {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([anti_mystery_dipole_inversion[e415], anti_mystery_dipole_inversion[e425], anti_mystery_dipole_inversion[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Circle> for CircleRotorAligningOrigin {
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
        if fail {
            let mut error = "Elements from Circle do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            circle.group0(),
            // e415, e425, e435
            Simd32x3::from([circle[e415], circle[e425], circle[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for CircleRotorAligningOrigin {
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
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for CircleRotorAligningOrigin {
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
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotor> for CircleRotorAligningOrigin {
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
        if fail {
            let mut error = "Elements from CircleRotor do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            circle_rotor.group0(),
            // e415, e425, e435
            Simd32x3::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435]]),
            // e235, e315, e125, e12345
            circle_rotor.group2(),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for CircleRotorAligningOrigin {
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
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([circle_rotor_at_infinity[e415], circle_rotor_at_infinity[e425], circle_rotor_at_infinity[e435]]),
            // e235, e315, e125, e12345
            circle_rotor_at_infinity.group1(),
        ));
    }
}

impl TryFrom<DualNum> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from DualNum do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
        ));
    }
}

impl TryFrom<Motor> for CircleRotorAligningOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([motor[e415], motor[e425], motor[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e12345]]),
        ));
    }
}

impl TryFrom<MotorAtInfinity> for CircleRotorAligningOrigin {
    type Error = String;
    fn try_from(motor_at_infinity: MotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorAtInfinity do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from MultiVector do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            multi_vector.group7(),
            // e415, e425, e435
            Simd32x3::from([multi_vector[e415], multi_vector[e425], multi_vector[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e12345]]),
        ));
    }
}

impl TryFrom<MysteryCircle> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryCircle do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryCircleRotor do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_circle_rotor[e12345]]),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from MysteryVersorEven do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_even[e12345]]),
        ));
    }
}

impl TryFrom<NullVersorEvenAtOrigin> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from NullVersorEvenAtOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEven> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from VersorEven do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even[e423], versor_even[e431], versor_even[e412]]),
            // e415, e425, e435
            Simd32x3::from([versor_even[e415], versor_even[e425], versor_even[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e12345]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for CircleRotorAligningOrigin {
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
        let el = versor_even_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_aligning_origin[e423], versor_even_aligning_origin[e431], versor_even_aligning_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e12345],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for CircleRotorAligningOrigin {
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
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from([versor_even_at_infinity[e415], versor_even_at_infinity[e425], versor_even_at_infinity[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_at_infinity[e235],
                versor_even_at_infinity[e315],
                versor_even_at_infinity[e125],
                versor_even_at_infinity[e12345],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for CircleRotorAligningOrigin {
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
        let el = versor_even_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from VersorEvenOnOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e12345]]),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for CircleRotorAligningOrigin {
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
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into CircleRotorAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412]]),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even_orthogonal_origin[e235], versor_even_orthogonal_origin[e315], versor_even_orthogonal_origin[e125], 0.0]),
        ));
    }
}
