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
// Total Implementations: 476
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         7       0       0
//  Average:        35      41       0
//  Maximum:       733     765       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         7       0       0
//  Average:        45      54       0
//  Maximum:       992    1024       0
impl std::ops::Add<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiDualNum> for MultiVector {
    fn add_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for MultiVector {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiLine> for MultiVector {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for MultiVector {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotor> for MultiVector {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], other[e4] + self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Infinity> for MultiVector {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar] + self[scalar], other[e12345] + self[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1] + self[e1], other[e2] + self[e2], other[e3] + self[e3], other[e4] + self[e4]]),
            // e5
            other[e5] + self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([other[e41] + self[e41], other[e42] + self[e42], other[e43] + self[e43], other[e45] + self[e45]]),
            // e15, e25, e35
            Simd32x3::from([other[e15] + self[e15], other[e25] + self[e25], other[e35] + self[e35]]),
            // e23, e31, e12
            Simd32x3::from([other[e23] + self[e23], other[e31] + self[e31], other[e12] + self[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415] + self[e415], other[e425] + self[e425], other[e435] + self[e435], other[e321] + self[e321]]),
            // e423, e431, e412
            Simd32x3::from([other[e423] + self[e423], other[e431] + self[e431], other[e412] + self[e412]]),
            // e235, e315, e125
            Simd32x3::from([other[e235] + self[e235], other[e315] + self[e315], other[e125] + self[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234] + self[e1234], other[e4235] + self[e4235], other[e4315] + self[e4315], other[e4125] + self[e4125]]),
            // e3215
            other[e3215] + self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryDipole> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryVersorEven> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<MysteryVersorOdd> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<SphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEven> for MultiVector {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] + other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] + other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] + other[e415], self[e425] + other[e425], self[e435] + other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] + other[e1], self[e2] + other[e2], self[e3] + other[e3], self[e4] + other[e4]]),
            // e5
            self[e5] + other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] + other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] + other[e423], self[e431] + other[e431], self[e412] + other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] + other[e235], self[e315] + other[e315], self[e125] + other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Add<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOdd> for MultiVector {
    fn add_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOddAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] + other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] + other[e4235], self[e4315] + other[e4315], self[e4125] + other[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] + other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] + other[e41], self[e42] + other[e42], self[e43] + other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] + other[e15], self[e25] + other[e25], self[e35] + other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] + other[e23], self[e31] + other[e31], self[e12] + other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] + other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] + other[e3215],
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       39        0
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       24       43        0
    //  no simd       26       52        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       68        0
    //    simd3        6        8        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       59       81        0
    //  no simd       80      112        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       68        0
    //    simd3        6        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       54       79        0
    //  no simd       72      104        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd3        5        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       32       52        0
    //  no simd       48       80        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        0
    //    simd3        5        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       40       63        0
    //  no simd       56       88        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       47        0
    //    simd3        4        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       34       57        0
    //  no simd       48       80        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       78        0
    //    simd3        5        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       67       90        0
    //  no simd       89      120        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       68        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       50       76        0
    //  no simd       62       96        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       63        0
    //    simd3        2        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       43       70        0
    //  no simd       50       87        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       49        0
    //    simd3        2        5        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       37       57        0
    //  no simd       44       76        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        6       25        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2       17        0
    //  no simd        2       34        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       15        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlatOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlatOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       20        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlatPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       60        0
    //    simd3        3        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       41       67        0
    //  no simd       50       84        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlector> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       49        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       24       54        0
    //  no simd       24       67        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlectorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlectorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       36        0
    //    simd3        1        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       26       48        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiLine> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiLine) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiLineOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       42        0
    //    simd3        5        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       34       54        0
    //  no simd       50       82        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd3        3        7        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       15       33        0
    //  no simd       24       56        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       34        0
    //    simd3        3        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       20       44        0
    //  no simd       32       67        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       60        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       33       65        0
    //  no simd       36       78        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMysteryDipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       51        0
    //    simd3        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       30       56        0
    //  no simd       34       68        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiPlane> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiPlane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       45        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       20       50        0
    //  no simd       20       63        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiPlaneOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiPlaneOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       50        0
    //    simd3        1        3        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       29       56        0
    //  no simd       34       71        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiSphereOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       49        0
    //    simd3        4        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       36       59        0
    //  no simd       50       82        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       26        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       24       40        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Circle> for MultiVector {
    fn bitxor_assign(&mut self, other: Circle) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       30        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       21       40        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       23        0
    //  no simd       15       28        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       12       24        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       13       28        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       15       35        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       27        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       25       41        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       22       41        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       14       29        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       24        0
    //  no simd       16       29        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       14       29        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       50       69        0
    //  no simd       54       80        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Dipole> for MultiVector {
    fn bitxor_assign(&mut self, other: Dipole) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       49        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       34       51        0
    //  no simd       34       56        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       44        0
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       49        0
    //  no simd       33       60        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       45        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       27       47        0
    //  no simd       27       52        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       57       76        0
    //  no simd       64       90        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       55        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       41       58        0
    //  no simd       44       66        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       52        0
    //    simd3        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       39       57        0
    //  no simd       41       68        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       49        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       31       51        0
    //  no simd       31       56        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       25       40        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       64        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       47       69        0
    //  no simd       51       80        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       17       32        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       60        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       43       65        0
    //  no simd       47       76        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       33        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd2        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       22        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlatOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       17       36        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       10       28        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlatPointAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatPointAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       25       44        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Flector> for MultiVector {
    fn bitxor_assign(&mut self, other: Flector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       12       30        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlectorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: FlectorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        6       22        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlectorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Horizon> for MultiVector {
    fn bitxor_assign(&mut self, other: Horizon) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       16        0
    //  no simd        0       30        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Infinity> for MultiVector {
    fn bitxor_assign(&mut self, other: Infinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       13       28        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Line> for MultiVector {
    fn bitxor_assign(&mut self, other: Line) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        4       16        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<LineAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: LineAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<LineOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: LineOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       32        0
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       25       45        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Motor> for MultiVector {
    fn bitxor_assign(&mut self, other: Motor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       24        0
    //  no simd       12       36        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MotorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: MotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: MotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      153        0
    //    simd3       13       14        0
    //    simd4       10       12        0
    // Totals...
    // yes simd      155      179        0
    //  no simd      211      243        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryCircle> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryCircle) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd        7       18        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       28        0
    //  no simd       12       40        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryDipole> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryDipole) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       18       46        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       61        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       34       66        0
    //  no simd       37       79        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryVersorEven) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       40        0
    //    simd3        3        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       26       50        0
    //  no simd       38       73        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        4       16        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullCircleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullCircleAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       10       26        0
    //  no simd       10       32        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullDipoleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       12       28        0
    //  no simd       12       34        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullSphereAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullSphereAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd3        1        2        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        7       24        0
    //  no simd       12       43        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullVersorEvenAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       34        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Origin> for MultiVector {
    fn bitxor_assign(&mut self, other: Origin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       12        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Plane> for MultiVector {
    fn bitxor_assign(&mut self, other: Plane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<PlaneOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       52        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       37       60        0
    //  no simd       49       80        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<RoundPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPoint) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd3        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        8       29        0
    //  no simd        8       44        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<RoundPointAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPointAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Sphere> for MultiVector {
    fn bitxor_assign(&mut self, other: Sphere) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        1        8        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<SphereAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: SphereAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<SphereOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: SphereOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       79        0
    //    simd3        5        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       68       91        0
    //  no simd       90      121        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEven) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd3        2        5        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       38       58        0
    //  no simd       45       77        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       69        0
    //    simd3        3        4        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       51       77        0
    //  no simd       63       97        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd3        2        5        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       25       45        0
    //  no simd       32       64        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAtOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       27        0
    //    simd3        1        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       25       49        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       70        0
    //    simd3        5        6        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       58       81        0
    //  no simd       77      108        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       74        0
    //    simd3        6        8        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       66       88        0
    //  no simd       90      122        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       60        0
    //    simd3        5        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       48       71        0
    //  no simd       64       96        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       72        0
    //    simd3        6        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       58       83        0
    //  no simd       76      108        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiCircleOnOrigin> for MultiVector {
    fn from(from_anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_anti_circle_on_origin[e41], from_anti_circle_on_origin[e42], from_anti_circle_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_on_origin.group1(),
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

impl From<AntiCircleRotor> for MultiVector {
    fn from(from_anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_anti_circle_rotor[e41], from_anti_circle_rotor[e42], from_anti_circle_rotor[e43], from_anti_circle_rotor[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_anti_circle_rotor[e15], from_anti_circle_rotor[e25], from_anti_circle_rotor[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_anti_circle_rotor[e23], from_anti_circle_rotor[e31], from_anti_circle_rotor[e12]]),
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

impl From<AntiCircleRotorAligningOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([
                from_anti_circle_rotor_aligning_origin[e41],
                from_anti_circle_rotor_aligning_origin[e42],
                from_anti_circle_rotor_aligning_origin[e43],
                0.0,
            ]),
            // e15, e25, e35
            Simd32x3::from([
                from_anti_circle_rotor_aligning_origin[e15],
                from_anti_circle_rotor_aligning_origin[e25],
                from_anti_circle_rotor_aligning_origin[e35],
            ]),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin.group1(),
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

impl From<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_aligning_origin_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([
                from_anti_circle_rotor_aligning_origin_at_infinity[e15],
                from_anti_circle_rotor_aligning_origin_at_infinity[e25],
                from_anti_circle_rotor_aligning_origin_at_infinity[e35],
            ]),
            // e23, e31, e12
            from_anti_circle_rotor_aligning_origin_at_infinity.group0(),
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

impl From<AntiCircleRotorAtInfinity> for MultiVector {
    fn from(from_anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_circle_rotor_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([
                from_anti_circle_rotor_at_infinity[e15],
                from_anti_circle_rotor_at_infinity[e25],
                from_anti_circle_rotor_at_infinity[e35],
            ]),
            // e23, e31, e12
            Simd32x3::from([
                from_anti_circle_rotor_at_infinity[e23],
                from_anti_circle_rotor_at_infinity[e31],
                from_anti_circle_rotor_at_infinity[e12],
            ]),
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

impl From<AntiCircleRotorOnOrigin> for MultiVector {
    fn from(from_anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_circle_rotor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_anti_circle_rotor_on_origin[e41], from_anti_circle_rotor_on_origin[e42], from_anti_circle_rotor_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_circle_rotor_on_origin.group1(),
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

impl From<AntiDipoleInversion> for MultiVector {
    fn from(from_anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion[e1],
                from_anti_dipole_inversion[e2],
                from_anti_dipole_inversion[e3],
                from_anti_dipole_inversion[e4],
            ]),
            // e5
            from_anti_dipole_inversion[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion.group1(),
            // e423, e431, e412
            from_anti_dipole_inversion.group0(),
            // e235, e315, e125
            Simd32x3::from([from_anti_dipole_inversion[e235], from_anti_dipole_inversion[e315], from_anti_dipole_inversion[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionAtInfinity> for MultiVector {
    fn from(from_anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion_at_infinity[e1],
                from_anti_dipole_inversion_at_infinity[e2],
                from_anti_dipole_inversion_at_infinity[e3],
                0.0,
            ]),
            // e5
            from_anti_dipole_inversion_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_dipole_inversion_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_anti_dipole_inversion_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_dipole_inversion_on_origin[e1],
                from_anti_dipole_inversion_on_origin[e2],
                from_anti_dipole_inversion_on_origin[e3],
                from_anti_dipole_inversion_on_origin[e4],
            ]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_dipole_inversion_on_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([
                from_anti_dipole_inversion_on_origin[e423],
                from_anti_dipole_inversion_on_origin[e431],
                from_anti_dipole_inversion_on_origin[e412],
            ]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_dipole_inversion_orthogonal_origin[e4]]),
            // e5
            from_anti_dipole_inversion_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                from_anti_dipole_inversion_orthogonal_origin[e415],
                from_anti_dipole_inversion_orthogonal_origin[e425],
                from_anti_dipole_inversion_orthogonal_origin[e435],
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from([
                from_anti_dipole_inversion_orthogonal_origin[e423],
                from_anti_dipole_inversion_orthogonal_origin[e431],
                from_anti_dipole_inversion_orthogonal_origin[e412],
            ]),
            // e235, e315, e125
            Simd32x3::from([
                from_anti_dipole_inversion_orthogonal_origin[e235],
                from_anti_dipole_inversion_orthogonal_origin[e315],
                from_anti_dipole_inversion_orthogonal_origin[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleOnOrigin> for MultiVector {
    fn from(from_anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_dipole_on_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([from_anti_dipole_on_origin[e423], from_anti_dipole_on_origin[e431], from_anti_dipole_on_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDualNum> for MultiVector {
    fn from(from_anti_dual_num: AntiDualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_dual_num[scalar], 0.0]),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_dual_num[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlatOrigin> for MultiVector {
    fn from(from_anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flat_origin[e321]]),
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

impl From<AntiFlatPoint> for MultiVector {
    fn from(from_anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flat_point[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([from_anti_flat_point[e235], from_anti_flat_point[e315], from_anti_flat_point[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(from_anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector[e1], from_anti_flector[e2], from_anti_flector[e3], 0.0]),
            // e5
            from_anti_flector[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flector[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([from_anti_flector[e235], from_anti_flector[e315], from_anti_flector[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlectorOnOrigin> for MultiVector {
    fn from(from_anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_flector_on_origin[e1], from_anti_flector_on_origin[e2], from_anti_flector_on_origin[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_flector_on_origin[e321]]),
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

impl From<AntiLine> for MultiVector {
    fn from(from_anti_line: AntiLine) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_anti_line.group1(),
            // e23, e31, e12
            from_anti_line.group0(),
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

impl From<AntiLineOnOrigin> for MultiVector {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            from_anti_line_on_origin.group0(),
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

impl From<AntiMotor> for MultiVector {
    fn from(from_anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([from_anti_motor[e15], from_anti_motor[e25], from_anti_motor[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_anti_motor[e23], from_anti_motor[e31], from_anti_motor[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_anti_motor[e3215],
        );
    }
}

impl From<AntiMotorOnOrigin> for MultiVector {
    fn from(from_anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_motor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_anti_motor_on_origin[e23], from_anti_motor_on_origin[e31], from_anti_motor_on_origin[e12]]),
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

impl From<AntiMysteryCircleRotor> for MultiVector {
    fn from(from_anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_mystery_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_anti_mystery_circle_rotor[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_anti_mystery_circle_rotor[e23], from_anti_mystery_circle_rotor[e31], from_anti_mystery_circle_rotor[e12]]),
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

impl From<AntiMysteryDipoleInversion> for MultiVector {
    fn from(from_anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                from_anti_mystery_dipole_inversion[e1],
                from_anti_mystery_dipole_inversion[e2],
                from_anti_mystery_dipole_inversion[e3],
                0.0,
            ]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_anti_mystery_dipole_inversion.group0(),
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

impl From<AntiPlane> for MultiVector {
    fn from(from_anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_plane[e1], from_anti_plane[e2], from_anti_plane[e3], 0.0]),
            // e5
            from_anti_plane[e5],
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
    }
}

impl From<AntiPlaneOnOrigin> for MultiVector {
    fn from(from_anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_anti_plane_on_origin[e1], from_anti_plane_on_origin[e2], from_anti_plane_on_origin[e3], 0.0]),
            // e5
            0.0,
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
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_anti_scalar[e12345]]),
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

impl From<AntiSphereOnOrigin> for MultiVector {
    fn from(from_anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_anti_sphere_on_origin.group0(),
            // e5
            0.0,
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
    }
}

impl From<AntiVersorEvenOnOrigin> for MultiVector {
    fn from(from_anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_anti_versor_even_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_anti_versor_even_on_origin[e41], from_anti_versor_even_on_origin[e42], from_anti_versor_even_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_anti_versor_even_on_origin[e23], from_anti_versor_even_on_origin[e31], from_anti_versor_even_on_origin[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_anti_versor_even_on_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<Circle> for MultiVector {
    fn from(from_circle: Circle) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            from_circle.group1(),
            // e423, e431, e412
            from_circle.group0(),
            // e235, e315, e125
            from_circle.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAligningOrigin> for MultiVector {
    fn from(from_circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([from_circle_aligning_origin[e415], from_circle_aligning_origin[e425], from_circle_aligning_origin[e435], 0.0]),
            // e423, e431, e412
            from_circle_aligning_origin.group0(),
            // e235, e315, e125
            from_circle_aligning_origin.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtInfinity> for MultiVector {
    fn from(from_circle_at_infinity: CircleAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            from_circle_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_circle_at_infinity.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtOrigin> for MultiVector {
    fn from(from_circle_at_origin: CircleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_circle_at_origin.group0(),
            // e235, e315, e125
            from_circle_at_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleOnOrigin> for MultiVector {
    fn from(from_circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([from_circle_on_origin[e415], from_circle_on_origin[e425], from_circle_on_origin[e435], 0.0]),
            // e423, e431, e412
            from_circle_on_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleOrthogonalOrigin> for MultiVector {
    fn from(from_circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([0.0, 0.0, 0.0, from_circle_orthogonal_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([from_circle_orthogonal_origin[e423], from_circle_orthogonal_origin[e431], from_circle_orthogonal_origin[e412]]),
            // e235, e315, e125
            from_circle_orthogonal_origin.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(from_circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor[e12345]]),
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
            from_circle_rotor.group1(),
            // e423, e431, e412
            from_circle_rotor.group0(),
            // e235, e315, e125
            Simd32x3::from([from_circle_rotor[e235], from_circle_rotor[e315], from_circle_rotor[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOrigin> for MultiVector {
    fn from(from_circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin[e12345]]),
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
            Simd32x4::from([
                from_circle_rotor_aligning_origin[e415],
                from_circle_rotor_aligning_origin[e425],
                from_circle_rotor_aligning_origin[e435],
                0.0,
            ]),
            // e423, e431, e412
            from_circle_rotor_aligning_origin.group0(),
            // e235, e315, e125
            Simd32x3::from([
                from_circle_rotor_aligning_origin[e235],
                from_circle_rotor_aligning_origin[e315],
                from_circle_rotor_aligning_origin[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(from_circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_aligning_origin_at_infinity[e12345]]),
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
            Simd32x4::from([
                from_circle_rotor_aligning_origin_at_infinity[e415],
                from_circle_rotor_aligning_origin_at_infinity[e425],
                from_circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                from_circle_rotor_aligning_origin_at_infinity[e235],
                from_circle_rotor_aligning_origin_at_infinity[e315],
                from_circle_rotor_aligning_origin_at_infinity[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAtInfinity> for MultiVector {
    fn from(from_circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_at_infinity[e12345]]),
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
            from_circle_rotor_at_infinity.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([from_circle_rotor_at_infinity[e235], from_circle_rotor_at_infinity[e315], from_circle_rotor_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorOnOrigin> for MultiVector {
    fn from(from_circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_circle_rotor_on_origin[e12345]]),
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
            Simd32x4::from([from_circle_rotor_on_origin[e415], from_circle_rotor_on_origin[e425], from_circle_rotor_on_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([from_circle_rotor_on_origin[e423], from_circle_rotor_on_origin[e431], from_circle_rotor_on_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Dipole> for MultiVector {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole[e41], from_dipole[e42], from_dipole[e43], from_dipole[e45]]),
            // e15, e25, e35
            from_dipole.group2(),
            // e23, e31, e12
            Simd32x3::from([from_dipole[e23], from_dipole[e31], from_dipole[e12]]),
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

impl From<DipoleAligningOrigin> for MultiVector {
    fn from(from_dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_aligning_origin.group0(),
            // e15, e25, e35
            from_dipole_aligning_origin.group1(),
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

impl From<DipoleAtInfinity> for MultiVector {
    fn from(from_dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_dipole_at_infinity[e45]]),
            // e15, e25, e35
            from_dipole_at_infinity.group1(),
            // e23, e31, e12
            Simd32x3::from([from_dipole_at_infinity[e23], from_dipole_at_infinity[e31], from_dipole_at_infinity[e12]]),
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

impl From<DipoleAtOrigin> for MultiVector {
    fn from(from_dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole_at_origin[e41], from_dipole_at_origin[e42], from_dipole_at_origin[e43], 0.0]),
            // e15, e25, e35
            from_dipole_at_origin.group1(),
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

impl From<DipoleInversion> for MultiVector {
    fn from(from_dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole_inversion[e41], from_dipole_inversion[e42], from_dipole_inversion[e43], from_dipole_inversion[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_dipole_inversion[e15], from_dipole_inversion[e25], from_dipole_inversion[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_dipole_inversion[e23], from_dipole_inversion[e31], from_dipole_inversion[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion[e1234], from_dipole_inversion[e4235], from_dipole_inversion[e4315], from_dipole_inversion[e4125]]),
            // e3215
            from_dipole_inversion[e3215],
        );
    }
}

impl From<DipoleInversionAligningOrigin> for MultiVector {
    fn from(from_dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_aligning_origin.group0(),
            // e15, e25, e35
            Simd32x3::from([
                from_dipole_inversion_aligning_origin[e15],
                from_dipole_inversion_aligning_origin[e25],
                from_dipole_inversion_aligning_origin[e35],
            ]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                from_dipole_inversion_aligning_origin[e1234],
                from_dipole_inversion_aligning_origin[e4235],
                from_dipole_inversion_aligning_origin[e4315],
                from_dipole_inversion_aligning_origin[e4125],
            ]),
            // e3215
            from_dipole_inversion_aligning_origin[e3215],
        );
    }
}

impl From<DipoleInversionAtInfinity> for MultiVector {
    fn from(from_dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_dipole_inversion_at_infinity[e45]]),
            // e15, e25, e35
            from_dipole_inversion_at_infinity.group1(),
            // e23, e31, e12
            Simd32x3::from([from_dipole_inversion_at_infinity[e23], from_dipole_inversion_at_infinity[e31], from_dipole_inversion_at_infinity[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                0.0,
                from_dipole_inversion_at_infinity[e4235],
                from_dipole_inversion_at_infinity[e4315],
                from_dipole_inversion_at_infinity[e4125],
            ]),
            // e3215
            from_dipole_inversion_at_infinity[e3215],
        );
    }
}

impl From<DipoleInversionAtOrigin> for MultiVector {
    fn from(from_dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole_inversion_at_origin[e41], from_dipole_inversion_at_origin[e42], from_dipole_inversion_at_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([from_dipole_inversion_at_origin[e15], from_dipole_inversion_at_origin[e25], from_dipole_inversion_at_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_at_origin[e3215],
        );
    }
}

impl From<DipoleInversionOnOrigin> for MultiVector {
    fn from(from_dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_inversion_on_origin.group0(),
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
            from_dipole_inversion_on_origin.group1(),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([
                from_dipole_inversion_orthogonal_origin[e41],
                from_dipole_inversion_orthogonal_origin[e42],
                from_dipole_inversion_orthogonal_origin[e43],
                0.0,
            ]),
            // e15, e25, e35
            Simd32x3::from([
                from_dipole_inversion_orthogonal_origin[e15],
                from_dipole_inversion_orthogonal_origin[e25],
                from_dipole_inversion_orthogonal_origin[e35],
            ]),
            // e23, e31, e12
            from_dipole_inversion_orthogonal_origin.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_dipole_inversion_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_dipole_inversion_orthogonal_origin[e3215],
        );
    }
}

impl From<DipoleOnOrigin> for MultiVector {
    fn from(from_dipole_on_origin: DipoleOnOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            from_dipole_on_origin.group0(),
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

impl From<DipoleOrthogonalOrigin> for MultiVector {
    fn from(from_dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_dipole_orthogonal_origin[e41], from_dipole_orthogonal_origin[e42], from_dipole_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35
            from_dipole_orthogonal_origin.group2(),
            // e23, e31, e12
            from_dipole_orthogonal_origin.group1(),
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

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_dual_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_dual_num[e4]]),
            // e5
            0.0,
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
    }
}

impl From<FlatOrigin> for MultiVector {
    fn from(from_flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flat_origin[e45]]),
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

impl From<FlatPoint> for MultiVector {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flat_point[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_flat_point[e15], from_flat_point[e25], from_flat_point[e35]]),
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

impl From<FlatPointAtInfinity> for MultiVector {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            from_flat_point_at_infinity.group0(),
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

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flector[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_flector[e15], from_flector[e25], from_flector[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_flector[e4235], from_flector[e4315], from_flector[e4125]]),
            // e3215
            from_flector[e3215],
        );
    }
}

impl From<FlectorAtInfinity> for MultiVector {
    fn from(from_flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([from_flector_at_infinity[e15], from_flector_at_infinity[e25], from_flector_at_infinity[e35]]),
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
            from_flector_at_infinity[e3215],
        );
    }
}

impl From<FlectorOnOrigin> for MultiVector {
    fn from(from_flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flector_on_origin[e45]]),
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
            Simd32x4::from([0.0, from_flector_on_origin[e4235], from_flector_on_origin[e4315], from_flector_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            from_horizon[e3215],
        );
    }
}

impl From<Infinity> for MultiVector {
    fn from(from_infinity: Infinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_infinity[e5],
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
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([from_line[e415], from_line[e425], from_line[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineAtInfinity> for MultiVector {
    fn from(from_line_at_infinity: LineAtInfinity) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            from_line_at_infinity.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineOnOrigin> for MultiVector {
    fn from(from_line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from([from_line_on_origin[e415], from_line_on_origin[e425], from_line_on_origin[e435], 0.0]),
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

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([from_motor[e415], from_motor[e425], from_motor[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([from_motor[e235], from_motor[e315], from_motor[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorAtInfinity> for MultiVector {
    fn from(from_motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            from_motor_at_infinity[e5],
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
            Simd32x3::from([from_motor_at_infinity[e235], from_motor_at_infinity[e315], from_motor_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorOnOrigin> for MultiVector {
    fn from(from_motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_motor_on_origin[e12345]]),
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
            Simd32x4::from([from_motor_on_origin[e415], from_motor_on_origin[e425], from_motor_on_origin[e435], 0.0]),
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

impl From<MysteryCircle> for MultiVector {
    fn from(from_mystery_circle: MysteryCircle) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            from_mystery_circle.group0(),
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

impl From<MysteryCircleRotor> for MultiVector {
    fn from(from_mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_circle_rotor[e12345]]),
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
            from_mystery_circle_rotor.group0(),
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

impl From<MysteryDipole> for MultiVector {
    fn from(from_mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_mystery_dipole[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_mystery_dipole[e23], from_mystery_dipole[e31], from_mystery_dipole[e12]]),
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

impl From<MysteryDipoleInversion> for MultiVector {
    fn from(from_mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_mystery_dipole_inversion[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_mystery_dipole_inversion[e23], from_mystery_dipole_inversion[e31], from_mystery_dipole_inversion[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_dipole_inversion[e4235], from_mystery_dipole_inversion[e4315], from_mystery_dipole_inversion[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryVersorEven> for MultiVector {
    fn from(from_mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_mystery_versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_mystery_versor_even[e1], from_mystery_versor_even[e2], from_mystery_versor_even[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_mystery_versor_even.group1(),
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

impl From<MysteryVersorOdd> for MultiVector {
    fn from(from_mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_mystery_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_mystery_versor_odd[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([from_mystery_versor_odd[e23], from_mystery_versor_odd[e31], from_mystery_versor_odd[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_mystery_versor_odd[e4235], from_mystery_versor_odd[e4315], from_mystery_versor_odd[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<NullCircleAtOrigin> for MultiVector {
    fn from(from_null_circle_at_origin: NullCircleAtOrigin) -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            from_null_circle_at_origin.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<NullDipoleAtOrigin> for MultiVector {
    fn from(from_null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_null_dipole_at_origin[e41], from_null_dipole_at_origin[e42], from_null_dipole_at_origin[e43], 0.0]),
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

impl From<NullDipoleInversionAtOrigin> for MultiVector {
    fn from(from_null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([
                from_null_dipole_inversion_at_origin[e41],
                from_null_dipole_inversion_at_origin[e42],
                from_null_dipole_inversion_at_origin[e43],
                0.0,
            ]),
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
            Simd32x4::from([from_null_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullSphereAtOrigin> for MultiVector {
    fn from(from_null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_null_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullVersorEvenAtOrigin> for MultiVector {
    fn from(from_null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_null_versor_even_at_origin[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([from_null_versor_even_at_origin[e423], from_null_versor_even_at_origin[e431], from_null_versor_even_at_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_origin[e4]]),
            // e5
            0.0,
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
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane[e4235], from_plane[e4315], from_plane[e4125]]),
            // e3215
            from_plane[e3215],
        );
    }
}

impl From<PlaneOnOrigin> for MultiVector {
    fn from(from_plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_plane_on_origin[e4235], from_plane_on_origin[e4315], from_plane_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<RoundPoint> for MultiVector {
    fn from(from_round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_round_point.group0(),
            // e5
            from_round_point[e5],
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
    }
}

impl From<RoundPointAtOrigin> for MultiVector {
    fn from(from_round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_round_point_at_origin[e4]]),
            // e5
            from_round_point_at_origin[e5],
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
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_scalar[scalar], 0.0]),
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

impl From<Sphere> for MultiVector {
    fn from(from_sphere: Sphere) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere[e1234], from_sphere[e4235], from_sphere[e4315], from_sphere[e4125]]),
            // e3215
            from_sphere[e3215],
        );
    }
}

impl From<SphereAtOrigin> for MultiVector {
    fn from(from_sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_sphere_at_origin[e3215],
        );
    }
}

impl From<SphereOnOrigin> for MultiVector {
    fn from(from_sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
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
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_sphere_on_origin[e1234], from_sphere_on_origin[e4235], from_sphere_on_origin[e4315], from_sphere_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEven> for MultiVector {
    fn from(from_versor_even: VersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even[e12345]]),
            // e1, e2, e3, e4
            from_versor_even.group3(),
            // e5
            from_versor_even[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even.group1(),
            // e423, e431, e412
            Simd32x3::from([from_versor_even[e423], from_versor_even[e431], from_versor_even[e412]]),
            // e235, e315, e125
            Simd32x3::from([from_versor_even[e235], from_versor_even[e315], from_versor_even[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAligningOrigin> for MultiVector {
    fn from(from_versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_versor_even_aligning_origin[e4]]),
            // e5
            from_versor_even_aligning_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                from_versor_even_aligning_origin[e415],
                from_versor_even_aligning_origin[e425],
                from_versor_even_aligning_origin[e435],
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from([from_versor_even_aligning_origin[e423], from_versor_even_aligning_origin[e431], from_versor_even_aligning_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([from_versor_even_aligning_origin[e235], from_versor_even_aligning_origin[e315], from_versor_even_aligning_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtInfinity> for MultiVector {
    fn from(from_versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([from_versor_even_at_infinity[e1], from_versor_even_at_infinity[e2], from_versor_even_at_infinity[e3], 0.0]),
            // e5
            from_versor_even_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            from_versor_even_at_infinity.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([from_versor_even_at_infinity[e235], from_versor_even_at_infinity[e315], from_versor_even_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtOrigin> for MultiVector {
    fn from(from_versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_versor_even_at_origin[e4]]),
            // e5
            from_versor_even_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([from_versor_even_at_origin[e423], from_versor_even_at_origin[e431], from_versor_even_at_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([from_versor_even_at_origin[e235], from_versor_even_at_origin[e315], from_versor_even_at_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenOnOrigin> for MultiVector {
    fn from(from_versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, from_versor_even_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_versor_even_on_origin[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([from_versor_even_on_origin[e415], from_versor_even_on_origin[e425], from_versor_even_on_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([from_versor_even_on_origin[e423], from_versor_even_on_origin[e431], from_versor_even_on_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for MultiVector {
    fn from(from_versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            from_versor_even_orthogonal_origin.group2(),
            // e5
            from_versor_even_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_versor_even_orthogonal_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([
                from_versor_even_orthogonal_origin[e423],
                from_versor_even_orthogonal_origin[e431],
                from_versor_even_orthogonal_origin[e412],
            ]),
            // e235, e315, e125
            Simd32x3::from([
                from_versor_even_orthogonal_origin[e235],
                from_versor_even_orthogonal_origin[e315],
                from_versor_even_orthogonal_origin[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(from_versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([from_versor_odd[e41], from_versor_odd[e42], from_versor_odd[e43], from_versor_odd[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_versor_odd[e15], from_versor_odd[e25], from_versor_odd[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_versor_odd[e23], from_versor_odd[e31], from_versor_odd[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd[e1234], from_versor_odd[e4235], from_versor_odd[e4315], from_versor_odd[e4125]]),
            // e3215
            from_versor_odd[e3215],
        );
    }
}

impl From<VersorOddAtInfinity> for MultiVector {
    fn from(from_versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_versor_odd_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([from_versor_odd_at_infinity[e15], from_versor_odd_at_infinity[e25], from_versor_odd_at_infinity[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_versor_odd_at_infinity[e23], from_versor_odd_at_infinity[e31], from_versor_odd_at_infinity[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, from_versor_odd_at_infinity[e4235], from_versor_odd_at_infinity[e4315], from_versor_odd_at_infinity[e4125]]),
            // e3215
            from_versor_odd_at_infinity[e3215],
        );
    }
}

impl From<VersorOddOrthogonalOrigin> for MultiVector {
    fn from(from_versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([from_versor_odd_orthogonal_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([
                from_versor_odd_orthogonal_origin[e41],
                from_versor_odd_orthogonal_origin[e42],
                from_versor_odd_orthogonal_origin[e43],
                0.0,
            ]),
            // e15, e25, e35
            Simd32x3::from([from_versor_odd_orthogonal_origin[e15], from_versor_odd_orthogonal_origin[e25], from_versor_odd_orthogonal_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from([from_versor_odd_orthogonal_origin[e23], from_versor_odd_orthogonal_origin[e31], from_versor_odd_orthogonal_origin[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([from_versor_odd_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            from_versor_odd_orthogonal_origin[e3215],
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      117      147        0
    //    simd2        5        6        0
    //    simd3        7        7        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      132      163        0
    //  no simd      160      192        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      184      217        0
    //    simd2       10       10        0
    //    simd3       24       24        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      229      262        0
    //  no simd      320      353        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      182      212        0
    //    simd2        9       10        0
    //    simd3       20       20        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      218      249        0
    //  no simd      288      320        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      113      143        0
    //    simd2        6        7        0
    //    simd3       17       17        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      140      171        0
    //  no simd      192      224        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      115      148        0
    //    simd2        7        7        0
    //    simd3       21       21        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      151      184        0
    //  no simd      224      257        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      119      149        0
    //    simd2        6        7        0
    //    simd3       11       11        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      143      174        0
    //  no simd      192      224        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      290      322        0
    //    simd2        4        4        0
    //    simd3       38       38        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      341      373        0
    //  no simd      448      480        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      208      240        0
    //    simd2        4        4        0
    //    simd3       28       28        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      245      277        0
    //  no simd      320      352        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      138      170        0
    //    simd2        4        4        0
    //    simd3       18       18        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      166      198        0
    //  no simd      224      256        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      220      252        0
    //    simd3       28       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      252      284        0
    //  no simd      320      352        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       86        0
    //    simd2        1        1        0
    //    simd3        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       73      101        0
    //  no simd       96      132        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       23        0
    //    simd2        1        2        0
    //    simd3        3        7        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       11       38        0
    //  no simd       36       72        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for MultiVector {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    //    simd3        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       52        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       87        0
    //    simd2        1        1        0
    //    simd3       11       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       73      103        0
    //  no simd       96      135        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      134      166        0
    //    simd2        4        4        0
    //    simd3       22       22        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      164      196        0
    //  no simd      224      256        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlector> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       88        0
    //    simd2        3        4        0
    //    simd3        8        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       71      102        0
    //  no simd       96      128        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      111      141        0
    //    simd2        5        6        0
    //    simd3       13       13        0
    // Totals...
    // yes simd      129      160        0
    //  no simd      160      192        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for MultiVector {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       78        0
    //    simd2        2        3        0
    //    simd3        4        4        0
    // Totals...
    // yes simd       54       85        0
    //  no simd       64       96        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      154        0
    //    simd2        7        8        0
    //    simd3       22       22        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      158      189        0
    //  no simd      224      256        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       80        0
    //    simd2        3        4        0
    //    simd3        8        8        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       65       96        0
    //  no simd       96      128        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       89        0
    //    simd2        4        4        0
    //    simd3       12       12        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       79      112        0
    //  no simd      128      161        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      130      162        0
    //    simd2        4        4        0
    //    simd3       14       14        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      151      183        0
    //  no simd      192      224        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       55       88        0
    //    simd2        3        3        0
    //    simd3        9        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       69      102        0
    //  no simd       96      129        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlane> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       78        0
    //    simd2        2        3        0
    //    simd3        4        4        0
    // Totals...
    // yes simd       54       85        0
    //  no simd       64       96        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       57        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       86        0
    //    simd2        3        3        0
    //    simd3        7        7        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       67      100        0
    //  no simd       96      129        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiSphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      128      158        0
    //    simd2        7        8        0
    //    simd3       14       14        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      159      190        0
    //  no simd      224      256        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      202      234        0
    //    simd2        1        1        0
    //    simd3       24       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      230      262        0
    //  no simd      288      320        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Circle> for MultiVector {
    fn mul_assign(&mut self, other: Circle) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      196      228        0
    //    simd3       20       20        0
    // Totals...
    // yes simd      216      248        0
    //  no simd      256      288        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      127      159        0
    //    simd2        1        1        0
    //    simd3       17       17        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      148      180        0
    //  no simd      192      224        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      118      144        0
    //    simd3       14       16        0
    // Totals...
    // yes simd      132      160        0
    //  no simd      160      192        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      127      159        0
    //    simd3       11       11        0
    // Totals...
    // yes simd      138      170        0
    //  no simd      160      192        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      128      154        0
    //    simd2        1        1        0
    //    simd3       18       20        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      149      177        0
    //  no simd      192      224        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      218      250        0
    //    simd2        1        1        0
    //    simd3       28       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      251      283        0
    //  no simd      320      352        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      212      244        0
    //    simd3       24       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      237      269        0
    //  no simd      288      320        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      137      169        0
    //    simd3       17       17        0
    //    simd4        1        1        0
    // Totals...
    // yes simd      155      187        0
    //  no simd      192      224        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      143      175        0
    //    simd2        1        1        0
    //    simd3       21       21        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      169      201        0
    //  no simd      224      256        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      147      179        0
    //    simd3       15       15        0
    // Totals...
    // yes simd      162      194        0
    //  no simd      192      224        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      182      215        0
    //    simd2        9        9        0
    //    simd3       20       20        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      218      251        0
    //  no simd      288      321        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for MultiVector {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      107      137        0
    //    simd2        6        6        0
    //    simd3       15       16        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      135      166        0
    //  no simd      192      225        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      113      146        0
    //    simd2        6        6        0
    //    simd3       17       17        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      140      173        0
    //  no simd      192      225        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      132        0
    //    simd2        5        6        0
    //    simd3       11       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      124      153        0
    //  no simd      160      192        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      274      306        0
    //    simd2       11       11        0
    //    simd3       36       36        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      332      364        0
    //  no simd      448      480        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      212      244        0
    //    simd2        8        8        0
    //    simd3       28       28        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      258      290        0
    //  no simd      352      384        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      196      228        0
    //    simd2        7        7        0
    //    simd3       30       30        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      238      270        0
    //  no simd      320      352        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      129      156        0
    //    simd2        7        8        0
    //    simd3       19       20        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      161      190        0
    //  no simd      224      256        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      142      174        0
    //    simd2        4        4        0
    //    simd3       14       14        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      168      200        0
    //  no simd      224      256        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      200      230        0
    //    simd2       10       11        0
    //    simd3       24       24        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      241      272        0
    //  no simd      320      352        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       80        0
    //    simd2        3        3        0
    //    simd3        5        9        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       66       98        0
    //  no simd       97      137        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      180      210        0
    //    simd2        8        9        0
    //    simd3       16       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      207      238        0
    //  no simd      256      288        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       35        0
    //    simd3        3        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       46        0
    //  no simd       32       71        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd2        0        2        0
    //    simd3        0        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       50        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlatOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       80        0
    //    simd2        3        3        0
    //    simd3       11       13        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       68       97        0
    //  no simd       96      129        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       63        0
    //    simd2        2        3        0
    //    simd3        8        9        0
    // Totals...
    // yes simd       54       75        0
    //  no simd       72       96        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPointAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: FlatPointAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      130      162        0
    //    simd2        4        4        0
    //    simd3       22       22        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      161      193        0
    //  no simd      224      256        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       55       74        0
    //    simd2        3        4        0
    //    simd3       13       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       72       93        0
    //  no simd      104      128        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: FlectorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       92        0
    //    simd3        8        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71      103        0
    //  no simd       96      128        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       11        0
    //    simd2        0        1        0
    //    simd3        2        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       21        0
    //  no simd        8       44        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Horizon> for MultiVector {
    fn mul_assign(&mut self, other: Horizon) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       11        0
    //    simd2        0        2        0
    //    simd3        2        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        4       23        0
    //  no simd        8       49        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Infinity> for MultiVector {
    fn mul_assign(&mut self, other: Infinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      121      153        0
    //    simd3       13       13        0
    // Totals...
    // yes simd      134      166        0
    //  no simd      160      192        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       69        0
    //    simd3        8        9        0
    // Totals...
    // yes simd       56       78        0
    //  no simd       72       96        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<LineAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: LineAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       84        0
    //    simd3        4        4        0
    // Totals...
    // yes simd       56       88        0
    //  no simd       64       96        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<LineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: LineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      150      182        0
    //    simd3       22       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      174      206        0
    //  no simd      224      256        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       82        0
    //    simd3       13       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       75       97        0
    //  no simd      104      128        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: MotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72      104        0
    //    simd3        8        8        0
    // Totals...
    // yes simd       80      112        0
    //  no simd       96      128        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: MotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      610      642        0
    //    simd2       16       16        0
    //    simd3       78       78        0
    //    simd4       29       29        0
    // Totals...
    // yes simd      733      765        0
    //  no simd      992     1024        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       90        0
    //    simd2        1        1        0
    //    simd3        8        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       70      102        0
    //  no simd       96      128        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircle> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircle) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      110        0
    //    simd2        1        1        0
    //    simd3       12       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       94      126        0
    //  no simd      128      160        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       87        0
    //    simd2        3        3        0
    //    simd3        8        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       68      101        0
    //  no simd       96      129        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      126      158        0
    //    simd2        3        3        0
    //    simd3       16       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      148      180        0
    //  no simd      192      224        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      143      172        0
    //    simd2        4        4        0
    //    simd3       19       20        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      170      200        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorEven> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorEven) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      122      148        0
    //    simd2        4        4        0
    //    simd3       22       24        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      155      183        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       63       87        0
    //    simd3        3        3        0
    // Totals...
    // yes simd       66       90        0
    //  no simd       72       96        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullCircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullCircleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       69        0
    //    simd2        2        3        0
    //    simd3        3        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       56       78        0
    //  no simd       73       96        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       82        0
    //    simd2        3        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       75       97        0
    //  no simd      105      128        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       24        0
    //  no simd        8       40        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullSphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74       98        0
    //    simd3        6        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       83      107        0
    //  no simd      104      128        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullVersorEvenAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd2        0        2        0
    //    simd3        1        4        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        6       28        0
    //  no simd        8       53        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Origin> for MultiVector {
    fn mul_assign(&mut self, other: Origin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       63       95        0
    //    simd2        1        1        0
    //    simd3        9        9        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       74      106        0
    //  no simd       96      128        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       84        0
    //    simd3        4        4        0
    // Totals...
    // yes simd       56       88        0
    //  no simd       64       96        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       94        0
    //    simd2        3        3        0
    //    simd3       12       12        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       83      115        0
    //  no simd      128      160        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPoint> for MultiVector {
    fn mul_assign(&mut self, other: RoundPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       29        0
    //    simd3        4        8        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       12       41        0
    //  no simd       32       69        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPointAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: RoundPointAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76      108        0
    //    simd2        2        2        0
    //    simd3       12       12        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       93      125        0
    //  no simd      128      160        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for MultiVector {
    fn mul_assign(&mut self, other: Sphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       33        0
    //    simd2        1        2        0
    //    simd3        4        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       14       46        0
    //  no simd       32       73        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       97        0
    //    simd2        1        1        0
    //    simd3        7        7        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       75      107        0
    //  no simd       96      128        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      302      334        0
    //    simd2        4        4        0
    //    simd3       38       38        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      358      390        0
    //  no simd      480      512        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEven> for MultiVector {
    fn mul_assign(&mut self, other: VersorEven) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      254      286        0
    //    simd3       26       26        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      285      317        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      211      240        0
    //    simd2        4        4        0
    //    simd3       31       32        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      256      286        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      154      186        0
    //    simd3       18       18        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      176      208        0
    //  no simd      224      256        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      170      202        0
    //    simd3       14       14        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      187      219        0
    //  no simd      224      256        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      234      266        0
    //    simd2        4        4        0
    //    simd3       26       26        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      272      304        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      270      302        0
    //    simd2       12       12        0
    //    simd3       42       42        0
    //    simd4       15       15        0
    // Totals...
    // yes simd      339      371        0
    //  no simd      480      512        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      192      224        0
    //    simd2        8        8        0
    //    simd3       36       36        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      245      277        0
    //  no simd      352      384        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      208      238        0
    //    simd2       11       12        0
    //    simd3       26       26        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      256      287        0
    //  no simd      352      384        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn neg(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e5
            self[e5] * -1.0,
            // e41, e42, e43, e45
            self.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group4() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group5() * Simd32x3::from(-1.0),
            // e415, e425, e435, e321
            self.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            self.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            self.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            self.group9() * Simd32x4::from(-1.0),
            // e3215
            self[e3215] * -1.0,
        );
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       22        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiDualNum> for MultiVector {
    fn sub_assign(&mut self, other: AntiDualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiLine> for MultiVector {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11        0        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Infinity> for MultiVector {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryDipole> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryVersorEven> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<MysteryVersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<SphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEven> for MultiVector {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar], self[e12345] - other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], self[e4] - other[e4]]),
            // e5
            self[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415] - other[e415], self[e425] - other[e425], self[e435] - other[e435], self[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1] - other[e1], self[e2] - other[e2], self[e3] - other[e3], self[e4] - other[e4]]),
            // e5
            self[e5] - other[e5],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self[e415], self[e425], self[e435], self[e321] - other[e321]]),
            // e423, e431, e412
            Simd32x3::from([self[e423] - other[e423], self[e431] - other[e431], self[e412] - other[e412]]),
            // e235, e315, e125
            Simd32x3::from([self[e235] - other[e235], self[e315] - other[e315], self[e125] - other[e125]]),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e3215],
        );
    }
}
impl std::ops::Sub<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOddAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41], self[e42], self[e43], self[e45] - other[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234], self[e4235] - other[e4235], self[e4315] - other[e4315], self[e4125] - other[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12        0        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self[scalar] - other[scalar], self[e12345]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self[e41] - other[e41], self[e42] - other[e42], self[e43] - other[e43], self[e45]]),
            // e15, e25, e35
            Simd32x3::from([self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]),
            // e23, e31, e12
            Simd32x3::from([self[e23] - other[e23], self[e31] - other[e31], self[e12] - other[e12]]),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self[e1234] - other[e1234], self[e4235], self[e4315], self[e4125]]),
            // e3215
            self[e3215] - other[e3215],
        );
    }
}
