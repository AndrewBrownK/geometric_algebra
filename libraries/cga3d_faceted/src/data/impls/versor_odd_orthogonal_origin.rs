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
// Total Implementations: 426
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       2       0
//  Average:        16      20       0
//  Maximum:       265     296       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       7       0
//  Average:        18      24       0
//  Maximum:       352     384       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<Circle> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<DualNum> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Horizon> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<Infinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235], other[e4315], other[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            crate::swizzle!(other.group1(), 0, 1, 2).extend_to_4(self[e3215]),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            crate::swizzle!(other.group0(), 0, 1, 2).extend_to_4(self[e3215]),
        );
    }
}
impl std::ops::Add<RoundPoint> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Add<Sphere> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]),
        );
    }
}
impl std::ops::Add<VersorEven> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], other[e3215] + self[e3215]]),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[scalar] + self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12], other[e3215] + self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35], other[e1234] + self[e1234]]),
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       10       25        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       45        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       46        0
    //  no simd       33       49        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       27       42        0
    //  no simd       30       45        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       18       33        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       21       37        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       37       53        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       24       43        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       25        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       18       33        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       22       34        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       11        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       13        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       11        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       18       37        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       10       25        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       19       34        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        9       21        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       22        0
    //  no simd        9       25        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       11       29        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       11       30        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       19        0
    //  no simd        8       22        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for VersorOddOrthogonalOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       22        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       11       26        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       34        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorOddOrthogonalOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       19        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       18        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       13        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       17        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorOddOrthogonalOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       19        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       13        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       30        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       22       37        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       16       28        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       25        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       13       28        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       31        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       24       34        0
    //  no simd       27       42        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       25        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       21       33        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       17       29        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       15       30        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       21       39        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       30        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       19       37        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorOddOrthogonalOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for VersorOddOrthogonalOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for VersorOddOrthogonalOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorOddOrthogonalOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       72        0
    //    simd3        6        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       58       83        0
    //  no simd       76      108        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       13        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       16        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       12       30        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       12       28        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for VersorOddOrthogonalOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       18       34        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        4       16        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorOddOrthogonalOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       38       54        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       20       29        0
    //  no simd       26       35        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       25       44        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       16       28        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       39        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       28       41        0
    //  no simd       31       47        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       42        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       41       54        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       41        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       39        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       35       47        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_anti_circle_on_origin[e41], from_anti_circle_on_origin[e42], from_anti_circle_on_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_circle_on_origin[e23], from_anti_circle_on_origin[e31], from_anti_circle_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin[e41],
                from_anti_circle_rotor_aligning_origin[e42],
                from_anti_circle_rotor_aligning_origin[e43],
                from_anti_circle_rotor_aligning_origin[scalar],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin[e23],
                from_anti_circle_rotor_aligning_origin[e31],
                from_anti_circle_rotor_aligning_origin[e12],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin[e15],
                from_anti_circle_rotor_aligning_origin[e25],
                from_anti_circle_rotor_aligning_origin[e35],
                0.0,
            ]),
        );
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_circle_rotor_aligning_origin_at_infinity[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin_at_infinity[e23],
                from_anti_circle_rotor_aligning_origin_at_infinity[e31],
                from_anti_circle_rotor_aligning_origin_at_infinity[e12],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin_at_infinity[e15],
                from_anti_circle_rotor_aligning_origin_at_infinity[e25],
                from_anti_circle_rotor_aligning_origin_at_infinity[e35],
                0.0,
            ]),
        );
    }
}

impl From<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_circle_rotor_on_origin.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_circle_rotor_on_origin[e23], from_anti_circle_rotor_on_origin[e31], from_anti_circle_rotor_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_dual_num[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_dual_num[e1234]]),
        );
    }
}

impl From<AntiLine> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_line[e23], from_anti_line[e31], from_anti_line[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([from_anti_line[e15], from_anti_line[e25], from_anti_line[e35], 0.0]),
        );
    }
}

impl From<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_line_on_origin[e23], from_anti_line_on_origin[e31], from_anti_line_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotor> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_motor[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_motor[e23], from_anti_motor[e31], from_anti_motor[e12], from_anti_motor[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([from_anti_motor[e15], from_anti_motor[e25], from_anti_motor[e35], 0.0]),
        );
    }
}

impl From<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_motor_on_origin[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_motor_on_origin[e23], from_anti_motor_on_origin[e31], from_anti_motor_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            from_anti_versor_even_on_origin.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([from_anti_versor_even_on_origin[e23], from_anti_versor_even_on_origin[e31], from_anti_versor_even_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_versor_even_on_origin[e1234]]),
        );
    }
}

impl From<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_dipole_at_origin[e41], from_dipole_at_origin[e42], from_dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([from_dipole_at_origin[e15], from_dipole_at_origin[e25], from_dipole_at_origin[e35], 0.0]),
        );
    }
}

impl From<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_dipole_inversion_at_origin[e41], from_dipole_inversion_at_origin[e42], from_dipole_inversion_at_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_dipole_inversion_at_origin[e3215]]),
            // e15, e25, e35, e1234
            from_dipole_inversion_at_origin.group1(),
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                from_dipole_inversion_orthogonal_origin[e41],
                from_dipole_inversion_orthogonal_origin[e42],
                from_dipole_inversion_orthogonal_origin[e43],
                0.0,
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                from_dipole_inversion_orthogonal_origin[e23],
                from_dipole_inversion_orthogonal_origin[e31],
                from_dipole_inversion_orthogonal_origin[e12],
                from_dipole_inversion_orthogonal_origin[e3215],
            ]),
            // e15, e25, e35, e1234
            from_dipole_inversion_orthogonal_origin.group2(),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_dipole_orthogonal_origin[e41], from_dipole_orthogonal_origin[e42], from_dipole_orthogonal_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([from_dipole_orthogonal_origin[e23], from_dipole_orthogonal_origin[e31], from_dipole_orthogonal_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([from_dipole_orthogonal_origin[e15], from_dipole_orthogonal_origin[e25], from_dipole_orthogonal_origin[e35], 0.0]),
        );
    }
}

impl From<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([from_flat_point_at_infinity[e15], from_flat_point_at_infinity[e25], from_flat_point_at_infinity[e35], 0.0]),
        );
    }
}

impl From<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_flector_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([from_flector_at_infinity[e15], from_flector_at_infinity[e25], from_flector_at_infinity[e35], 0.0]),
        );
    }
}

impl From<Horizon> for VersorOddOrthogonalOrigin {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_horizon[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([from_null_dipole_at_origin[e41], from_null_dipole_at_origin[e42], from_null_dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                from_null_dipole_inversion_at_origin[e41],
                from_null_dipole_inversion_at_origin[e42],
                from_null_dipole_inversion_at_origin[e43],
                0.0,
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, from_null_dipole_inversion_at_origin[e1234]]),
        );
    }
}

impl From<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, from_null_sphere_at_origin[e1234]]),
        );
    }
}

impl From<Scalar> for VersorOddOrthogonalOrigin {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_scalar[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, from_sphere_at_origin[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, from_sphere_at_origin[e1234]]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      104      120        0
    //  no simd      116      132        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       92      108        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       95      111        0
    //  no simd      104      120        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       68       84        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      140        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      134      150        0
    //  no simd      164      180        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      100        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      116      132        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      104      120        0
    //  no simd      116      132        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       32       48        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       18        0
    //  no simd        8       24        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       47        0
    //  no simd       32       56        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       44       60        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       80       96        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      104      120        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       92      108        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       68       84        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      112        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      116      132        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       92      108        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       95      111        0
    //  no simd      104      120        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       68       84        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       80       96        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       68       84        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      104      120        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       68       84        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       68       84        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      140      156        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      146      162        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      113      129        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      104      120        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      107      123        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      104      120        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       32       50        0
    //  no simd       32       56        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       92      108        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        8       32        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       74       90        0
    //  no simd       80       96        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       56       72        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       80       96        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       36       48        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      220      250        0
    //    simd2       11       12        0
    //    simd3       26       26        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      265      296        0
    //  no simd      352      384        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       35       51        0
    //  no simd       44       60        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       68       84        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       68       84        0
    //  no simd       80       96        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       36       48        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       20        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
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
impl std::ops::Mul<Origin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       32       48        0
    //  no simd       44       60        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       17        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       19        0
    //  no simd        8       25        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       45       60        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       21        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       22        0
    //  no simd        8       25        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      140        0
    //    simd4       13       13        0
    // Totals...
    // yes simd      137      153        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      107      123        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      100        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       95      111        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      113      129        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      140      156        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      149      165        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      124        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      113      129        0
    //  no simd      128      144        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      116        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      107      123        0
    //  no simd      128      144        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn neg(self) -> Self::Output {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, e1234
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for VersorOddOrthogonalOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10        1        0
    //  no simd       10        4        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        1        0
    //  no simd        7        4        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]) * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiDualNum> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<Circle> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9        1        0
    //  no simd        9        4        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       11        2        0
    //  no simd       11        8        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        8        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        8        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<DualNum> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<FlatPoint> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<Flector> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<Horizon> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Horizon> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<Infinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       12        8        0
    //  no simd       12       25        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
            Simd32x4::from([self[e1234] - other[e1234], other[e4235], other[e4315], other[e4125]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self[e3215]]),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        8        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        8        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for VersorOddOrthogonalOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<RoundPoint> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<Scalar> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
        );
    }
}
impl std::ops::Sub<Sphere> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        2        4        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23], self[e31], self[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23], self[e31], self[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15], self[e25], self[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorEven> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], other[e12345]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorOddOrthogonalOrigin {
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
            Simd32x2::from([self[scalar], 0.0]),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([self[e15], self[e25], self[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23], self[e31], self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e423, e431, e412
            Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other[e235], other[e315], other[e125]]) * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], 0.0, 0.0, 0.0]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12        2        0
    //  no simd       12        8        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8        2        0
    //  no simd        8        8        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41], self[e42], self[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], other[e45]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], self[e3215] - other[e3215]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for VersorOddOrthogonalOrigin {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[scalar] - other[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12], self[e3215] - other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35], self[e1234] - other[e1234]]),
        );
    }
}

impl TryFrom<AntiCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43], anti_circle_rotor[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_at_infinity[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([anti_circle_rotor_at_infinity[e23], anti_circle_rotor_at_infinity[e31], anti_circle_rotor_at_infinity[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor_at_infinity[e15], anti_circle_rotor_at_infinity[e25], anti_circle_rotor_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([anti_mystery_circle_rotor[e23], anti_mystery_circle_rotor[e31], anti_mystery_circle_rotor[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from DipoleAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from DipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], dipole_inversion[e3215]]),
            // e15, e25, e35, e1234
            dipole_inversion.group2(),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_aligning_origin[e41], dipole_inversion_aligning_origin[e42], dipole_inversion_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_aligning_origin[e3215]]),
            // e15, e25, e35, e1234
            dipole_inversion_aligning_origin.group1(),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([
                dipole_inversion_at_infinity[e23],
                dipole_inversion_at_infinity[e31],
                dipole_inversion_at_infinity[e12],
                dipole_inversion_at_infinity[e3215],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_inversion_at_infinity[e15], dipole_inversion_at_infinity[e25], dipole_inversion_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_on_origin[e41], dipole_inversion_on_origin[e42], dipole_inversion_on_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from DipoleOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], 0.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<FlatPoint> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from FlatPoint do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
        ));
    }
}

impl TryFrom<Flector> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from Flector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from MultiVector do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
        ));
    }
}

impl TryFrom<MysteryDipole> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole: MysteryDipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipole do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([mystery_dipole[e23], mystery_dipole[e31], mystery_dipole[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_dipole_inversion: MysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
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
            let mut error = "Elements from MysteryDipoleInversion do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([mystery_dipole_inversion[e23], mystery_dipole_inversion[e31], mystery_dipole_inversion[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
        let el = mystery_versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([mystery_versor_odd[e23], mystery_versor_odd[e31], mystery_versor_odd[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Plane> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from Plane do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, plane[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Sphere> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from Sphere do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for VersorOddOrthogonalOrigin {
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
            let mut error = "Elements from SphereOnOrigin do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOdd> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from VersorOdd do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            versor_odd.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e3215]]),
            // e15, e25, e35, e1234
            versor_odd.group2(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for VersorOddOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from VersorOddAtInfinity do not fit into VersorOddOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[scalar]]),
            // e23, e31, e12, e3215
            Simd32x4::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12], versor_odd_at_infinity[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
        ));
    }
}
