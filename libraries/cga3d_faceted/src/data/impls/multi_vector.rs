use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 536
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       0       0
//  Average:        23      29       0
//  Maximum:       501     529       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         7       0       0
//  Average:        41      50       0
//  Maximum:       992    1024       0
impl std::ops::Add<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]])),
            // e5
            (self[e1] + other.group3()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]])),
            // e5
            (self[e1] + other.group3()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + swizzle!(other.group1(), 1, 2, 3, 0)),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + swizzle!(other.group1(), 1, 2, 3, 0)),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group2()[3])]),
            // e5
            (self[e1] + other.group0()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group2()[3])]),
            // e5
            (self[e1] + other.group0()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other[e321])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other[e321])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for MultiVector {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlector> for MultiVector {
    fn add_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLine> for MultiVector {
    fn add_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[e31]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[e31]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[0])]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryQuadNum> for MultiVector {
    fn add_assign(&mut self, other: AntiMysteryQuadNum) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[0])]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group0()[3]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlane> for MultiVector {
    fn add_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group0()[3]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiQuadNum) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNum> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNum) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] + other.group0()[0]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] + other.group0()[0]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[1])]),
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
            (self[e45] + other.group0()[0]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNumAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[1])]),
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
            (self[e45] + other.group0()[0]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNumOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[1]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Circle> for MultiVector {
    fn add_assign(&mut self, other: Circle) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotor> for MultiVector {
    fn add_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + other.group2()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Dipole> for MultiVector {
    fn add_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + other.group2()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
            // e15, e25, e35
            (self.group4() + other.group1()),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
            // e15, e25, e35
            (self.group4() + other.group1()),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + other.group1()),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + other.group1()),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] + other.group3()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] + other.group3()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group2()[0]),
                (self.group9()[2] + other.group2()[1]),
                (self.group9()[3] + other.group2()[2]),
            ]),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + other.group1()),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group2()[0]),
                (self.group9()[2] + other.group2()[1]),
                (self.group9()[3] + other.group2()[2]),
            ]),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
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
            (self.group9() + other.group1()),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
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
            (self.group9() + other.group1()),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + other.group0()),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + other.group2()),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + other.group2()),
            // e23, e31, e12
            (self.group5() + other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e45])]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e45])]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPoint> for MultiVector {
    fn add_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + other.group0()),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + other.group0()),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group1()[0]),
                (self.group9()[2] + other.group1()[1]),
                (self.group9()[3] + other.group1()[2]),
            ]),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group1()[0]),
                (self.group9()[2] + other.group1()[1]),
                (self.group9()[3] + other.group1()[2]),
            ]),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            (self[e45] + other.group0()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            (self[e45] + other.group0()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[0])]),
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[1]),
                (self.group9()[2] + other.group0()[2]),
                (self.group9()[3] + other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[0])]),
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[1]),
                (self.group9()[2] + other.group0()[2]),
                (self.group9()[3] + other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] + other[e3215]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] + other[e3215]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other[e5]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Infinity> for MultiVector {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other[e5]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group8() + other.group0()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group8() + other.group0()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[3]),
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
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[3]),
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
            (self.group8() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group0()[0]),
                (self.group6()[1] + other.group0()[1]),
                (self.group6()[2] + other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() + other.group0()),
            // e1, e2, e3, e4
            (self.group1() + other.group1()),
            // e5
            (self[e1] + other[e1]),
            // e41, e42, e43, e45
            (self.group3() + other.group3()),
            // e15, e25, e35
            (self.group4() + other.group4()),
            // e23, e31, e12
            (self.group5() + other.group5()),
            // e415, e425, e435, e321
            (self.group6() + other.group6()),
            // e423, e431, e412
            (self.group7() + other.group7()),
            // e235, e315, e125
            (self.group8() + other.group8()),
            // e1234, e4235, e4315, e4125
            (self.group9() + other.group9()),
            // e3215
            (self[e45] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() + other.group0()),
            // e1, e2, e3, e4
            (self.group1() + other.group1()),
            // e5
            (self[e1] + other[e1]),
            // e41, e42, e43, e45
            (self.group3() + other.group3()),
            // e15, e25, e35
            (self.group4() + other.group4()),
            // e23, e31, e12
            (self.group5() + other.group5()),
            // e415, e425, e435, e321
            (self.group6() + other.group6()),
            // e423, e431, e412
            (self.group7() + other.group7()),
            // e235, e315, e125
            (self.group8() + other.group8()),
            // e1234, e4235, e4315, e4125
            (self.group9() + other.group9()),
            // e3215
            (self[e45] + other[e45]),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryCircle> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e425])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for MultiVector {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e425])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipole> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group1()[0]),
                (self.group9()[2] + other.group1()[1]),
                (self.group9()[3] + other.group1()[2]),
            ]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for MultiVector {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group1()[0]),
                (self.group9()[2] + other.group1()[1]),
                (self.group9()[3] + other.group1()[2]),
            ]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: MysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryQuadNum> for MultiVector {
    fn add_assign(&mut self, other: MysteryQuadNum) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryVersorEven> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[1]),
                (self.group9()[2] + other.group0()[2]),
                (self.group9()[3] + other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryVersorOdd> for MultiVector {
    fn add_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[1]),
                (self.group9()[2] + other.group0()[2]),
                (self.group9()[3] + other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
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
            Simd32x4::from([(self.group9()[0] + other.group0()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other[e1234]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other[e1234]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e4])]),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e4])]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[0]),
                (self.group9()[2] + other.group0()[1]),
                (self.group9()[3] + other.group0()[2]),
            ]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[0]),
                (self.group9()[2] + other.group0()[1]),
                (self.group9()[3] + other.group0()[2]),
            ]),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[0]),
                (self.group9()[2] + other.group0()[1]),
                (self.group9()[3] + other.group0()[2]),
            ]),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group0()[0]),
                (self.group9()[2] + other.group0()[1]),
                (self.group9()[3] + other.group0()[2]),
            ]),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: QuadNum) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNum> for MultiVector {
    fn add_assign(&mut self, other: QuadNum) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: QuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[2])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: QuadNumAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[2])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumAligningOriginAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: QuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[2])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[1])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: QuadNumAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[2])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] + other.group0()[0]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[1])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: QuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: QuadNumOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            (self[e1] + other[e2]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<RoundPoint> for MultiVector {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group0()),
            // e5
            (self[e1] + other[e2]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e5
            (self[e1] + other.group0()[1]),
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() + Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Sphere> for MultiVector {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() + Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e3215
            (self[e45] + other.group0()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[1]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[0]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] + other.group0()[1]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group0()[0]),
        );
        *self = addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() + swizzle!(other.group0(), 3, 0, 1, 2)),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<SphereOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() + swizzle!(other.group0(), 3, 0, 1, 2)),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() + other.group3()),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEven> for MultiVector {
    fn add_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() + other.group3()),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[3])]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAligningOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[3])]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] + other.group0()[1]),
                (self.group1()[1] + other.group0()[2]),
                (self.group1()[2] + other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] + other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() + other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenOnOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] + other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group1()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] + other.group1()[0]),
                (self.group6()[1] + other.group1()[1]),
                (self.group6()[2] + other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group2()),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() + other.group2()),
            // e5
            (self[e1] + other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] + other.group0()[3])]),
            // e423, e431, e412
            (self.group7() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] + other.group3()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOdd> for MultiVector {
    fn add_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() + Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] + other.group3()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group2()[0]),
                (self.group9()[2] + other.group2()[1]),
                (self.group9()[3] + other.group2()[2]),
            ]),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOddAtInfinity> for MultiVector {
    fn add_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] + other.group2()[0]),
                (self.group9()[2] + other.group2()[1]),
                (self.group9()[3] + other.group2()[2]),
            ]),
            // e3215
            (self[e45] + other.group2()[3]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] + other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() + Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() + Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] + other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] + other.group1()[3]),
        );
        *self = addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        2        5        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       26       48        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       54        0
    //    simd3        7       10        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       51       71        0
    //  no simd       80      112        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       58        0
    //    simd3        7       10        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       49       72        0
    //  no simd       72      104        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd3        5        8        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       29       49        0
    //  no simd       48       80        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd3        5        8        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       34       57        0
    //  no simd       56       88        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        4        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       54        0
    //  no simd       48       80        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       60        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       56       77        0
    //  no simd       89      120        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       54        0
    //    simd3        4        6        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       42       65        0
    //  no simd       62       92        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       50        0
    //    simd3        3        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       35       60        0
    //  no simd       50       84        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       40        0
    //    simd3        2        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       29       49        0
    //  no simd       45       71        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        6       20        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDipoleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlatOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       16        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlatPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       46        0
    //    simd3        4        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       33       56        0
    //  no simd       50       80        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlector> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       38        0
    //    simd3        1        4        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       44        0
    //  no simd       27       58        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiFlectorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        2        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       26       48        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiLine> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiLineOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       38        0
    //    simd3        5        8        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       31       51        0
    //  no simd       50       82        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       27        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       35       64        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       38        0
    //    simd3        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       24       48        0
    //  no simd       36       70        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd3        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       29        0
    //  no simd        8       40        0
    fn bitxor(self, other: AntiMysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryQuadNum> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiMysteryQuadNum) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       37        0
    //    simd3        3        5        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       22       45        0
    //  no simd       34       64        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiPlane> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       30        0
    //    simd3        3        8        0
    // Totals...
    // yes simd       14       38        0
    //  no simd       20       54        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiPlaneOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       27        0
    //  no simd       12       44        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNum> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNum) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        4       36        0
    fn bitxor(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn bitxor(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       26        0
    //    simd3        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       31        0
    //  no simd       10       42        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNumAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2       17        0
    //  no simd        2       34        0
    fn bitxor(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNumOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2       15        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       37        0
    //    simd3        2        5        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       46        0
    //  no simd       34       68        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiSphereOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       41        0
    //    simd3        4        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       50       82        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn bitxor(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Circle> for MultiVector {
    fn bitxor_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       26        0
    //  no simd       21       36        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       12       27        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       24        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       31        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       22       37        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       14       25        0
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       14       25        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<CircleRotorOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       44        0
    //    simd3        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       54       80        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Dipole> for MultiVector {
    fn bitxor_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       39        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       29       44        0
    //  no simd       34       56        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       30        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       33       56        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       33        0
    //    simd3        2        5        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       27       48        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       56        0
    //    simd3        3        6        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       49       66        0
    //  no simd       64       90        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       51        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       44       66        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       38        0
    //    simd3        2        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       41       64        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       41        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       25       44        0
    //  no simd       31       52        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       40        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       38       59        0
    //  no simd       51       76        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       17       32        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       44        0
    //    simd3        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       47       72        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<FlatOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       17       32        0
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
    //      f32        7       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        8       18        0
    //  no simd       10       24        0
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
    // Totals...
    // yes simd       25       38        0
    //  no simd       25       40        0
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
    // Totals...
    // yes simd       12       24        0
    //  no simd       12       26        0
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
    //      add/sub      mul      div
    // f32        6       17        0
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
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Horizon> for MultiVector {
    fn bitxor_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Infinity> for MultiVector {
    fn bitxor_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
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
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       12        0
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
    //      add/sub      mul      div
    // f32        5       12        0
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
    // Totals...
    // yes simd       23       35        0
    //  no simd       25       41        0
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
    // Totals...
    // yes simd       10       22        0
    //  no simd       12       28        0
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
    //      add/sub      mul      div
    // f32        6       13        0
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
    //      f32       99      117        0
    //    simd3       16       18        0
    //    simd4       16       18        0
    // Totals...
    // yes simd      131      153        0
    //  no simd      211      243        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryCircle> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryCircleRotor> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       26        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       12       29        0
    //  no simd       12       35        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryDipole> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       18       35        0
    //  no simd       18       41        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryDipoleInversion> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryQuadNum> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryQuadNum) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       40        0
    //    simd3        1        4        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       26       48        0
    //  no simd       40       68        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       26       44        0
    //  no simd       41       70        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        4       15        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullCircleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       10       24        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullDipoleAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullSphereAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       17        0
    //    simd3        1        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        7       22        0
    //  no simd       12       35        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       24        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Origin> for MultiVector {
    fn bitxor_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
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
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<PlaneOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd3        0        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       28        0
    //  no simd       12       40        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNum> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNum) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       28        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        9       31        0
    //  no simd        9       37        0
    fn bitxor(self, other: QuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNumAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNumAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       17        0
    fn bitxor(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNumAligningOriginAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       15        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3       17        0
    //  no simd        3       21        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNumAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNumAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1       11        0
    //  no simd        1       25        0
    fn bitxor(self, other: QuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNumOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNumOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       26        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       30        0
    //  no simd       11       39        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<QuadNumOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       49       80        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<RoundPoint> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       27        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        8       30        0
    //  no simd        8       36        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<RoundPointAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Sphere> for MultiVector {
    fn bitxor_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        4        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<SphereAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<SphereOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       78        0
    //  no simd       90      121        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEven> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       45        0
    //    simd3        2        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       32       53        0
    //  no simd       45       72        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAligningOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       55        0
    //    simd3        4        6        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       43       66        0
    //  no simd       63       93        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       36        0
    //    simd3        2        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       43        0
    //  no simd       32       59        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenAtOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       25       45        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenOnOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       56        0
    //    simd3        6        8        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       50       71        0
    //  no simd       77      108        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd       90      122        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        0
    //    simd3        5        8        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       42       65        0
    //  no simd       64       96        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       60        0
    //    simd3        6        8        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       49       74        0
    //  no simd       76      108        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}

impl From<AntiCircleOnOrigin> for MultiVector {
    fn from(anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12]]),
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
    fn from(anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43], anti_circle_rotor[e45]]),
            // e15, e25, e35
            Simd32x3::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12]]),
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
    fn from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_circle_rotor_aligning_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e41],
                anti_circle_rotor_aligning_origin[e42],
                anti_circle_rotor_aligning_origin[e43],
                0.0,
            ]),
            // e15, e25, e35
            Simd32x3::from([anti_circle_rotor_aligning_origin[e15], anti_circle_rotor_aligning_origin[e25], anti_circle_rotor_aligning_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor_aligning_origin[e23], anti_circle_rotor_aligning_origin[e31], anti_circle_rotor_aligning_origin[e12]]),
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
    fn from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_circle_rotor_aligning_origin_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([
                anti_circle_rotor_aligning_origin_at_infinity[e15],
                anti_circle_rotor_aligning_origin_at_infinity[e25],
                anti_circle_rotor_aligning_origin_at_infinity[e35],
            ]),
            // e23, e31, e12
            Simd32x3::from([
                anti_circle_rotor_aligning_origin_at_infinity[e23],
                anti_circle_rotor_aligning_origin_at_infinity[e31],
                anti_circle_rotor_aligning_origin_at_infinity[e12],
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

impl From<AntiCircleRotorAtInfinity> for MultiVector {
    fn from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_circle_rotor_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([anti_circle_rotor_at_infinity[e15], anti_circle_rotor_at_infinity[e25], anti_circle_rotor_at_infinity[e35]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor_at_infinity[e23], anti_circle_rotor_at_infinity[e31], anti_circle_rotor_at_infinity[e12]]),
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
    fn from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_circle_rotor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([anti_circle_rotor_on_origin[e41], anti_circle_rotor_on_origin[e42], anti_circle_rotor_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor_on_origin[e23], anti_circle_rotor_on_origin[e31], anti_circle_rotor_on_origin[e12]]),
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
    fn from(anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3], anti_dipole_inversion[e4]]),
            // e5
            anti_dipole_inversion[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435], anti_dipole_inversion[e321]]),
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion[e423], anti_dipole_inversion[e431], anti_dipole_inversion[e412]]),
            // e235, e315, e125
            Simd32x3::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionAtInfinity> for MultiVector {
    fn from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion_at_infinity[e1], anti_dipole_inversion_at_infinity[e2], anti_dipole_inversion_at_infinity[e3], 0.0]),
            // e5
            anti_dipole_inversion_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
                anti_dipole_inversion_at_infinity[e321],
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for MultiVector {
    fn from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                anti_dipole_inversion_on_origin[e1],
                anti_dipole_inversion_on_origin[e2],
                anti_dipole_inversion_on_origin[e3],
                anti_dipole_inversion_on_origin[e4],
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
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412]]),
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
    fn from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_orthogonal_origin[e4]]),
            // e5
            anti_dipole_inversion_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e415],
                anti_dipole_inversion_orthogonal_origin[e425],
                anti_dipole_inversion_orthogonal_origin[e435],
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from([
                anti_dipole_inversion_orthogonal_origin[e423],
                anti_dipole_inversion_orthogonal_origin[e431],
                anti_dipole_inversion_orthogonal_origin[e412],
            ]),
            // e235, e315, e125
            Simd32x3::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiDipoleOnOrigin> for MultiVector {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
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
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlatOrigin> for MultiVector {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
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
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
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
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
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
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlector> for MultiVector {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], 0.0]),
            // e5
            anti_flector[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([anti_flector[e235], anti_flector[e315], anti_flector[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<AntiFlectorOnOrigin> for MultiVector {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
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
    fn from(anti_line: AntiLine) -> Self {
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
            Simd32x3::from([anti_line[e15], anti_line[e25], anti_line[e35]]),
            // e23, e31, e12
            Simd32x3::from([anti_line[e23], anti_line[e31], anti_line[e12]]),
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
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
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
            Simd32x3::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12]]),
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
    fn from(anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_motor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([anti_motor[e15], anti_motor[e25], anti_motor[e35]]),
            // e23, e31, e12
            Simd32x3::from([anti_motor[e23], anti_motor[e31], anti_motor[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            anti_motor[e3215],
        );
    }
}

impl From<AntiMotorOnOrigin> for MultiVector {
    fn from(anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_motor_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12]]),
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
    fn from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_mystery_circle_rotor[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_mystery_circle_rotor[e23], anti_mystery_circle_rotor[e31], anti_mystery_circle_rotor[e12]]),
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
    fn from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_mystery_dipole_inversion[e1], anti_mystery_dipole_inversion[e2], anti_mystery_dipole_inversion[e3], 0.0]),
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
                anti_mystery_dipole_inversion[e415],
                anti_mystery_dipole_inversion[e425],
                anti_mystery_dipole_inversion[e435],
                anti_mystery_dipole_inversion[e321],
            ]),
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

impl From<AntiMysteryQuadNum> for MultiVector {
    fn from(anti_mystery_quad_num: AntiMysteryQuadNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_mystery_quad_num[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_quad_num[e45]]),
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

impl From<AntiPlane> for MultiVector {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], 0.0]),
            // e5
            anti_plane[e5],
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
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
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

impl From<AntiQuadNum> for MultiVector {
    fn from(anti_quad_num: AntiQuadNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_quad_num[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e45]]),
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
            Simd32x4::from([anti_quad_num[e1234], 0.0, 0.0, 0.0]),
            // e3215
            anti_quad_num[e3215],
        );
    }
}

impl From<AntiQuadNumAligningOrigin> for MultiVector {
    fn from(anti_quad_num_aligning_origin: AntiQuadNumAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_quad_num_aligning_origin[scalar], 0.0]),
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
            Simd32x4::from([anti_quad_num_aligning_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            anti_quad_num_aligning_origin[e3215],
        );
    }
}

impl From<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    fn from(anti_quad_num_aligning_origin_at_infinity: AntiQuadNumAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_quad_num_aligning_origin_at_infinity[scalar], 0.0]),
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
            anti_quad_num_aligning_origin_at_infinity[e3215],
        );
    }
}

impl From<AntiQuadNumAtInfinity> for MultiVector {
    fn from(anti_quad_num_at_infinity: AntiQuadNumAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_quad_num_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[e45]]),
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
            anti_quad_num_at_infinity[e3215],
        );
    }
}

impl From<AntiQuadNumOnOrigin> for MultiVector {
    fn from(anti_quad_num_on_origin: AntiQuadNumOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_quad_num_on_origin[scalar], 0.0]),
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
            Simd32x4::from([anti_quad_num_on_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<AntiQuadNumOrthogonalOrigin> for MultiVector {
    fn from(anti_quad_num_orthogonal_origin: AntiQuadNumOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e45]]),
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
            Simd32x4::from([anti_quad_num_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            anti_quad_num_orthogonal_origin[e3215],
        );
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, anti_scalar[e12345]]),
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
    fn from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], anti_sphere_on_origin[e4]]),
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
    fn from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([anti_versor_even_on_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([anti_versor_even_on_origin[e41], anti_versor_even_on_origin[e42], anti_versor_even_on_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_versor_even_on_origin[e23], anti_versor_even_on_origin[e31], anti_versor_even_on_origin[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([anti_versor_even_on_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<Circle> for MultiVector {
    fn from(circle: Circle) -> Self {
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
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e423, e431, e412
            Simd32x3::from([circle[e423], circle[e431], circle[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle[e235], circle[e315], circle[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAligningOrigin> for MultiVector {
    fn from(circle_aligning_origin: CircleAligningOrigin) -> Self {
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
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtInfinity> for MultiVector {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
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
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleAtOrigin> for MultiVector {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
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
            Simd32x3::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleOnOrigin> for MultiVector {
    fn from(circle_on_origin: CircleOnOrigin) -> Self {
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
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412]]),
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
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
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
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotor> for MultiVector {
    fn from(circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor[e12345]]),
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
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e423, e431, e412
            Simd32x3::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOrigin> for MultiVector {
    fn from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor_aligning_origin[e12345]]),
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
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([circle_rotor_aligning_origin[e423], circle_rotor_aligning_origin[e431], circle_rotor_aligning_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor_aligning_origin[e235], circle_rotor_aligning_origin[e315], circle_rotor_aligning_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor_aligning_origin_at_infinity[e12345]]),
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
                circle_rotor_aligning_origin_at_infinity[e415],
                circle_rotor_aligning_origin_at_infinity[e425],
                circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                circle_rotor_aligning_origin_at_infinity[e235],
                circle_rotor_aligning_origin_at_infinity[e315],
                circle_rotor_aligning_origin_at_infinity[e125],
            ]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorAtInfinity> for MultiVector {
    fn from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor_at_infinity[e12345]]),
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
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([circle_rotor_at_infinity[e235], circle_rotor_at_infinity[e315], circle_rotor_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<CircleRotorOnOrigin> for MultiVector {
    fn from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, circle_rotor_on_origin[e12345]]),
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
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([circle_rotor_on_origin[e423], circle_rotor_on_origin[e431], circle_rotor_on_origin[e412]]),
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
    fn from(dipole: Dipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], dipole[e45]]),
            // e15, e25, e35
            Simd32x3::from([dipole[e15], dipole[e25], dipole[e35]]),
            // e23, e31, e12
            Simd32x3::from([dipole[e23], dipole[e31], dipole[e12]]),
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
    fn from(dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], dipole_aligning_origin[e45]]),
            // e15, e25, e35
            Simd32x3::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35]]),
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
    fn from(dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35]]),
            // e23, e31, e12
            Simd32x3::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12]]),
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
    fn from(dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35]]),
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
    fn from(dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43], dipole_inversion[e45]]),
            // e15, e25, e35
            Simd32x3::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35]]),
            // e23, e31, e12
            Simd32x3::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([dipole_inversion[e1234], dipole_inversion[e4235], dipole_inversion[e4315], dipole_inversion[e4125]]),
            // e3215
            dipole_inversion[e3215],
        );
    }
}

impl From<DipoleInversionAligningOrigin> for MultiVector {
    fn from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
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
                dipole_inversion_aligning_origin[e41],
                dipole_inversion_aligning_origin[e42],
                dipole_inversion_aligning_origin[e43],
                dipole_inversion_aligning_origin[e45],
            ]),
            // e15, e25, e35
            Simd32x3::from([dipole_inversion_aligning_origin[e15], dipole_inversion_aligning_origin[e25], dipole_inversion_aligning_origin[e35]]),
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
                dipole_inversion_aligning_origin[e1234],
                dipole_inversion_aligning_origin[e4235],
                dipole_inversion_aligning_origin[e4315],
                dipole_inversion_aligning_origin[e4125],
            ]),
            // e3215
            dipole_inversion_aligning_origin[e3215],
        );
    }
}

impl From<DipoleInversionAtInfinity> for MultiVector {
    fn from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([dipole_inversion_at_infinity[e15], dipole_inversion_at_infinity[e25], dipole_inversion_at_infinity[e35]]),
            // e23, e31, e12
            Simd32x3::from([dipole_inversion_at_infinity[e23], dipole_inversion_at_infinity[e31], dipole_inversion_at_infinity[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, dipole_inversion_at_infinity[e4235], dipole_inversion_at_infinity[e4315], dipole_inversion_at_infinity[e4125]]),
            // e3215
            dipole_inversion_at_infinity[e3215],
        );
    }
}

impl From<DipoleInversionAtOrigin> for MultiVector {
    fn from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_inversion_at_origin[e41], dipole_inversion_at_origin[e42], dipole_inversion_at_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([dipole_inversion_at_origin[e15], dipole_inversion_at_origin[e25], dipole_inversion_at_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            dipole_inversion_at_origin[e3215],
        );
    }
}

impl From<DipoleInversionOnOrigin> for MultiVector {
    fn from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
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
                dipole_inversion_on_origin[e41],
                dipole_inversion_on_origin[e42],
                dipole_inversion_on_origin[e43],
                dipole_inversion_on_origin[e45],
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
            Simd32x4::from([
                dipole_inversion_on_origin[e1234],
                dipole_inversion_on_origin[e4235],
                dipole_inversion_on_origin[e4315],
                dipole_inversion_on_origin[e4125],
            ]),
            // e3215
            0.0,
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
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
                dipole_inversion_orthogonal_origin[e41],
                dipole_inversion_orthogonal_origin[e42],
                dipole_inversion_orthogonal_origin[e43],
                0.0,
            ]),
            // e15, e25, e35
            Simd32x3::from([
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
            ]),
            // e23, e31, e12
            Simd32x3::from([
                dipole_inversion_orthogonal_origin[e23],
                dipole_inversion_orthogonal_origin[e31],
                dipole_inversion_orthogonal_origin[e12],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([dipole_inversion_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            dipole_inversion_orthogonal_origin[e3215],
        );
    }
}

impl From<DipoleOnOrigin> for MultiVector {
    fn from(dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], dipole_on_origin[e45]]),
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
    fn from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12]]),
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
    fn from(flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_origin[e45]]),
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
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]]),
            // e15, e25, e35
            Simd32x3::from([flat_point[e15], flat_point[e25], flat_point[e35]]),
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
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
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
            Simd32x3::from([flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35]]),
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
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e15, e25, e35
            Simd32x3::from([flector[e15], flector[e25], flector[e35]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, flector[e4235], flector[e4315], flector[e4125]]),
            // e3215
            flector[e3215],
        );
    }
}

impl From<FlectorAtInfinity> for MultiVector {
    fn from(flector_at_infinity: FlectorAtInfinity) -> Self {
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
            Simd32x3::from([flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35]]),
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
            flector_at_infinity[e3215],
        );
    }
}

impl From<FlectorOnOrigin> for MultiVector {
    fn from(flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]]),
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
            Simd32x4::from([0.0, flector_on_origin[e4235], flector_on_origin[e4315], flector_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<Horizon> for MultiVector {
    fn from(horizon: Horizon) -> Self {
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
            horizon[e3215],
        );
    }
}

impl From<Infinity> for MultiVector {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            infinity[e5],
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
    fn from(line: Line) -> Self {
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
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([line[e235], line[e315], line[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineAtInfinity> for MultiVector {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
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
            Simd32x3::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<LineOnOrigin> for MultiVector {
    fn from(line_on_origin: LineOnOrigin) -> Self {
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
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
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
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, motor[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            motor[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([motor[e235], motor[e315], motor[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorAtInfinity> for MultiVector {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            motor_at_infinity[e5],
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
            Simd32x3::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<MotorOnOrigin> for MultiVector {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, motor_on_origin[e12345]]),
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
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
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
    fn from(mystery_circle: MysteryCircle) -> Self {
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
            Simd32x4::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435], mystery_circle[e321]]),
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
    fn from(mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, mystery_circle_rotor[e12345]]),
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
            Simd32x4::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435], mystery_circle_rotor[e321]]),
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
    fn from(mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([mystery_dipole[e23], mystery_dipole[e31], mystery_dipole[e12]]),
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
    fn from(mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_dipole_inversion[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([mystery_dipole_inversion[e23], mystery_dipole_inversion[e31], mystery_dipole_inversion[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, mystery_dipole_inversion[e4235], mystery_dipole_inversion[e4315], mystery_dipole_inversion[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<MysteryQuadNum> for MultiVector {
    fn from(mystery_quad_num: MysteryQuadNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, mystery_quad_num[e12345]]),
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
            Simd32x4::from([0.0, 0.0, 0.0, mystery_quad_num[e321]]),
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

impl From<MysteryVersorEven> for MultiVector {
    fn from(mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, mystery_versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435], mystery_versor_even[e321]]),
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
    fn from(mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([mystery_versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([mystery_versor_odd[e23], mystery_versor_odd[e31], mystery_versor_odd[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<NullCircleAtOrigin> for MultiVector {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
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
            Simd32x3::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412]]),
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
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
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
    fn from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([null_dipole_inversion_at_origin[e41], null_dipole_inversion_at_origin[e42], null_dipole_inversion_at_origin[e43], 0.0]),
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
            Simd32x4::from([null_dipole_inversion_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullSphereAtOrigin> for MultiVector {
    fn from(null_sphere_at_origin: NullSphereAtOrigin) -> Self {
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
            Simd32x4::from([null_sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}

impl From<NullVersorEvenAtOrigin> for MultiVector {
    fn from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_even_at_origin[e4]]),
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
            Simd32x3::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412]]),
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
    fn from(origin: Origin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, origin[e4]]),
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
    fn from(plane: Plane) -> Self {
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
            Simd32x4::from([0.0, plane[e4235], plane[e4315], plane[e4125]]),
            // e3215
            plane[e3215],
        );
    }
}

impl From<PlaneOnOrigin> for MultiVector {
    fn from(plane_on_origin: PlaneOnOrigin) -> Self {
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
            Simd32x4::from([0.0, plane_on_origin[e4235], plane_on_origin[e4315], plane_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<QuadNum> for MultiVector {
    fn from(quad_num: QuadNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, quad_num[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e4]]),
            // e5
            quad_num[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e321]]),
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

impl From<QuadNumAligningOrigin> for MultiVector {
    fn from(quad_num_aligning_origin: QuadNumAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, quad_num_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_aligning_origin[e4]]),
            // e5
            quad_num_aligning_origin[e5],
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

impl From<QuadNumAligningOriginAtInfinity> for MultiVector {
    fn from(quad_num_aligning_origin_at_infinity: QuadNumAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, quad_num_aligning_origin_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            quad_num_aligning_origin_at_infinity[e5],
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

impl From<QuadNumAtInfinity> for MultiVector {
    fn from(quad_num_at_infinity: QuadNumAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, quad_num_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            quad_num_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_at_infinity[e321]]),
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

impl From<QuadNumOnOrigin> for MultiVector {
    fn from(quad_num_on_origin: QuadNumOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, quad_num_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_on_origin[e4]]),
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

impl From<QuadNumOrthogonalOrigin> for MultiVector {
    fn from(quad_num_orthogonal_origin: QuadNumOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_orthogonal_origin[e4]]),
            // e5
            quad_num_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_orthogonal_origin[e321]]),
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

impl From<RoundPoint> for MultiVector {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e4]]),
            // e5
            round_point[e5],
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
    fn from(round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e4]]),
            // e5
            round_point_at_origin[e5],
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
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([scalar[scalar], 0.0]),
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
    fn from(sphere: Sphere) -> Self {
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
            Simd32x4::from([sphere[e1234], sphere[e4235], sphere[e4315], sphere[e4125]]),
            // e3215
            sphere[e3215],
        );
    }
}

impl From<SphereAtOrigin> for MultiVector {
    fn from(sphere_at_origin: SphereAtOrigin) -> Self {
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
            Simd32x4::from([sphere_at_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            sphere_at_origin[e3215],
        );
    }
}

impl From<SphereOnOrigin> for MultiVector {
    fn from(sphere_on_origin: SphereOnOrigin) -> Self {
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
            Simd32x4::from([sphere_on_origin[e1234], sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125]]),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEven> for MultiVector {
    fn from(versor_even: VersorEven) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, versor_even[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e4]]),
            // e5
            versor_even[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e423, e431, e412
            Simd32x3::from([versor_even[e423], versor_even[e431], versor_even[e412]]),
            // e235, e315, e125
            Simd32x3::from([versor_even[e235], versor_even[e315], versor_even[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAligningOrigin> for MultiVector {
    fn from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, versor_even_aligning_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e4]]),
            // e5
            versor_even_aligning_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([versor_even_aligning_origin[e423], versor_even_aligning_origin[e431], versor_even_aligning_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([versor_even_aligning_origin[e235], versor_even_aligning_origin[e315], versor_even_aligning_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtInfinity> for MultiVector {
    fn from(versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, versor_even_at_infinity[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
            // e5
            versor_even_at_infinity[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenAtOrigin> for MultiVector {
    fn from(versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e4]]),
            // e5
            versor_even_at_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorEvenOnOrigin> for MultiVector {
    fn from(versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, versor_even_on_origin[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412]]),
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
    fn from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e4],
            ]),
            // e5
            versor_even_orthogonal_origin[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e423, e431, e412
            Simd32x3::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412]]),
            // e235, e315, e125
            Simd32x3::from([versor_even_orthogonal_origin[e235], versor_even_orthogonal_origin[e315], versor_even_orthogonal_origin[e125]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}

impl From<VersorOdd> for MultiVector {
    fn from(versor_odd: VersorOdd) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([versor_odd[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[e45]]),
            // e15, e25, e35
            Simd32x3::from([versor_odd[e15], versor_odd[e25], versor_odd[e35]]),
            // e23, e31, e12
            Simd32x3::from([versor_odd[e23], versor_odd[e31], versor_odd[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([versor_odd[e1234], versor_odd[e4235], versor_odd[e4315], versor_odd[e4125]]),
            // e3215
            versor_odd[e3215],
        );
    }
}

impl From<VersorOddAtInfinity> for MultiVector {
    fn from(versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([versor_odd_at_infinity[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[e45]]),
            // e15, e25, e35
            Simd32x3::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35]]),
            // e23, e31, e12
            Simd32x3::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, versor_odd_at_infinity[e4235], versor_odd_at_infinity[e4315], versor_odd_at_infinity[e4125]]),
            // e3215
            versor_odd_at_infinity[e3215],
        );
    }
}

impl From<VersorOddOrthogonalOrigin> for MultiVector {
    fn from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([versor_odd_orthogonal_origin[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e41], versor_odd_orthogonal_origin[e42], versor_odd_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35
            Simd32x3::from([versor_odd_orthogonal_origin[e15], versor_odd_orthogonal_origin[e25], versor_odd_orthogonal_origin[e35]]),
            // e23, e31, e12
            Simd32x3::from([versor_odd_orthogonal_origin[e23], versor_odd_orthogonal_origin[e31], versor_odd_orthogonal_origin[e12]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([versor_odd_orthogonal_origin[e1234], 0.0, 0.0, 0.0]),
            // e3215
            versor_odd_orthogonal_origin[e3215],
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       87      105        0
    //    simd2        5        6        0
    //    simd3       17       21        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      112      135        0
    //  no simd      160      192        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      109      136        0
    //    simd2       10       12        0
    //    simd3       37       38        0
    //    simd4       20       20        0
    // Totals...
    // yes simd      176      206        0
    //  no simd      320      354        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      127      154        0
    //    simd2        9       10        0
    //    simd3       33       34        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      180      209        0
    //  no simd      288      320        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       80      110        0
    //    simd2        6        7        0
    //    simd3       24       24        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      117      148        0
    //  no simd      192      224        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       87        0
    //    simd2        7        9        0
    //    simd3       28       29        0
    //    simd4       17       17        0
    // Totals...
    // yes simd      113      142        0
    //  no simd      227      260        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       86        0
    //    simd2        6        7        0
    //    simd3       18       20        0
    //    simd4       16       16        0
    // Totals...
    // yes simd      102      129        0
    //  no simd      192      224        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      154      186        0
    //    simd2        4        4        0
    //    simd3       50       50        0
    //    simd4       34       34        0
    // Totals...
    // yes simd      242      274        0
    //  no simd      448      480        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      123        0
    //    simd2        4        4        0
    //    simd3       40       42        0
    //    simd4       24       24        0
    // Totals...
    // yes simd      164      193        0
    //  no simd      320      353        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       90        0
    //    simd2        4        4        0
    //    simd3       21       23        0
    //    simd4       23       23        0
    // Totals...
    // yes simd      109      140        0
    //  no simd      224      259        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       98      127        0
    //    simd3       38       39        0
    //    simd4       27       27        0
    // Totals...
    // yes simd      163      193        0
    //  no simd      320      352        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       44        0
    //    simd2        1        1        0
    //    simd3        9       14        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       47       70        0
    //  no simd       99      132        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       66        0
    //    simd2        1        1        0
    //    simd3       13       16        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       60       88        0
    //  no simd       96      136        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73      104        0
    //    simd2        4        4        0
    //    simd3       28       28        0
    //    simd4       15       15        0
    // Totals...
    // yes simd      120      151        0
    //  no simd      225      256        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlector> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd2        3        4        0
    //    simd3       10       12        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       43       64        0
    //  no simd       96      128        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiFlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73       91        0
    //    simd2        5        6        0
    //    simd3       23       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      103      126        0
    //  no simd      160      192        0
    fn mul(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for MultiVector {
    fn mul_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       54        0
    //    simd2        2        3        0
    //    simd3        8       12        0
    // Totals...
    // yes simd       46       69        0
    //  no simd       64       96        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       94        0
    //    simd2        7        8        0
    //    simd3       29       30        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      121      146        0
    //  no simd      228      256        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       40        0
    //    simd2        3        4        0
    //    simd3       12       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       45       68        0
    //  no simd       96      128        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       38        0
    //    simd2        4        6        0
    //    simd3       16       20        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       53       78        0
    //  no simd      128      166        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       97        0
    //    simd2        4        4        0
    //    simd3       23       25        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      109      137        0
    //  no simd      192      224        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd2        1        3        0
    //    simd3        4        8        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       11       27        0
    //  no simd       32       70        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryQuadNum> for MultiVector {
    fn mul_assign(&mut self, other: AntiMysteryQuadNum) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       44        0
    //    simd2        3        5        0
    //    simd3       11       13        0
    //    simd4        9       11        0
    // Totals...
    // yes simd       45       73        0
    //  no simd       97      137        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlane> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       54        0
    //    simd2        2        3        0
    //    simd3        8       12        0
    // Totals...
    // yes simd       46       69        0
    //  no simd       64       96        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiPlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       16        0
    //    simd2        3        5        0
    //    simd3       12       16        0
    //    simd4       11       15        0
    // Totals...
    // yes simd       36       52        0
    //  no simd       96      134        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNum> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNum) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd2        2        3        0
    //    simd3        8       12        0
    //    simd4        7       11        0
    // Totals...
    // yes simd       25       40        0
    //  no simd       64      100        0
    fn mul(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd2        1        2        0
    //    simd3        5        9        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       38        0
    //  no simd       32       64        0
    fn mul(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd2        2        4        0
    //    simd3        9       13        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       29       48        0
    //  no simd       67       99        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNumAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       18        0
    //    simd2        1        2        0
    //    simd3        3        7        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       11       35        0
    //  no simd       36       75        0
    fn mul(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNumOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd2        2        4        0
    //    simd3        8       12        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       64      102        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       51        0
    //    simd2        3        5        0
    //    simd3        9       11        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       45       77        0
    //  no simd       96      134        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiSphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       69        0
    //    simd2        7        8        0
    //    simd3       21       24        0
    //    simd4       25       25        0
    // Totals...
    // yes simd      100      126        0
    //  no simd      224      257        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      116      142        0
    //    simd2        1        1        0
    //    simd3       34       36        0
    //    simd4       17       17        0
    // Totals...
    // yes simd      168      196        0
    //  no simd      288      320        0
    fn mul(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Circle> for MultiVector {
    fn mul_assign(&mut self, other: Circle) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      116      136        0
    //    simd3       32       36        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      159      183        0
    //  no simd      256      288        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       86        0
    //    simd2        1        1        0
    //    simd3       25       27        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      100      128        0
    //  no simd      193      225        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       88      108        0
    //    simd3       20       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      111      135        0
    //  no simd      160      192        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      105        0
    //    simd3       17       21        0
    //    simd4        6        6        0
    // Totals...
    // yes simd      108      132        0
    //  no simd      160      192        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       82        0
    //    simd2        1        1        0
    //    simd3       24       28        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      101      125        0
    //  no simd      192      224        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      122      154        0
    //    simd2        1        1        0
    //    simd3       36       36        0
    //    simd4       22       22        0
    // Totals...
    // yes simd      181      213        0
    //  no simd      320      352        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      122      148        0
    //    simd3       34       36        0
    //    simd4       16       16        0
    // Totals...
    // yes simd      172      200        0
    //  no simd      288      320        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      112        0
    //    simd3       25       27        0
    //    simd4        8        8        0
    // Totals...
    // yes simd      118      147        0
    //  no simd      192      225        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       74      106        0
    //    simd2        1        1        0
    //    simd3       27       27        0
    //    simd4       17       17        0
    // Totals...
    // yes simd      119      151        0
    //  no simd      225      257        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       79        0
    //    simd3       21       24        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       93      122        0
    //  no simd      192      227        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<CircleRotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      106      124        0
    //    simd2        9       11        0
    //    simd3       36       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd      165      189        0
    //  no simd      288      322        0
    fn mul(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for MultiVector {
    fn mul_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       74        0
    //    simd2        6        8        0
    //    simd3       21       24        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       96      122        0
    //  no simd      192      226        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       71        0
    //    simd2        6        8        0
    //    simd3       27       31        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       99      122        0
    //  no simd      195      228        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78       96        0
    //    simd2        5        6        0
    //    simd3       20       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      106      129        0
    //  no simd      160      192        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      145      174        0
    //    simd2       11       11        0
    //    simd3       51       52        0
    //    simd4       32       32        0
    // Totals...
    // yes simd      239      269        0
    //  no simd      448      480        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      100      132        0
    //    simd2        8        8        0
    //    simd3       36       36        0
    //    simd4       32       32        0
    // Totals...
    // yes simd      176      208        0
    //  no simd      352      384        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      108        0
    //    simd2        7        7        0
    //    simd3       43       46        0
    //    simd4       23       23        0
    // Totals...
    // yes simd      158      184        0
    //  no simd      320      352        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       98        0
    //    simd2        7        8        0
    //    simd3       22       22        0
    //    simd4       19       19        0
    // Totals...
    // yes simd      116      147        0
    //  no simd      224      256        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       75        0
    //    simd2        4        4        0
    //    simd3       20       22        0
    //    simd4       27       27        0
    // Totals...
    // yes simd      102      128        0
    //  no simd      227      257        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      102      132        0
    //    simd2       10       11        0
    //    simd3       34       34        0
    //    simd4       24       24        0
    // Totals...
    // yes simd      170      201        0
    //  no simd      320      352        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd2        3        5        0
    //    simd3        6       11        0
    //    simd4       15       17        0
    // Totals...
    // yes simd       43       64        0
    //  no simd      103      142        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      124      142        0
    //    simd2        8        9        0
    //    simd3       32       36        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      169      192        0
    //  no simd      256      288        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       73        0
    //    simd2        3        5        0
    //    simd3       12       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       63       93        0
    //  no simd       96      130        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for MultiVector {
    fn mul_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       45        0
    //    simd2        2        3        0
    //    simd3       12       15        0
    // Totals...
    // yes simd       46       63        0
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
    //      f32       76      105        0
    //    simd2        4        4        0
    //    simd3       27       28        0
    //    simd4       15       15        0
    // Totals...
    // yes simd      122      152        0
    //  no simd      225      257        0
    fn mul(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd2        3        4        0
    //    simd3       14       14        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       67       90        0
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
    //      f32       27       46        0
    //    simd3       11       14        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       47       70        0
    //  no simd       96      128        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd2        0        1        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       22        0
    //  no simd        8       36        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Horizon> for MultiVector {
    fn mul_assign(&mut self, other: Horizon) {
        use crate::elements::*;
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
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       20        0
    //  no simd        8       37        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Infinity> for MultiVector {
    fn mul_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       71       92        0
    //    simd3       23       27        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       99      124        0
    //  no simd      160      193        0
    fn mul(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       47        0
    //    simd3       12       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       45       63        0
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
    //      f32       32       52        0
    //    simd3        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       42       66        0
    //  no simd       64       96        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<LineOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73      100        0
    //    simd3       28       28        0
    //    simd4       18       18        0
    // Totals...
    // yes simd      119      146        0
    //  no simd      229      256        0
    fn mul(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       72        0
    //    simd3       15       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       68       90        0
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
    //      f32       18       28        0
    //    simd3       10       12        0
    //    simd4       12       16        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       96      128        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MotorOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      288      314        0
    //    simd2       16       16        0
    //    simd3      116      118        0
    //    simd4       81       81        0
    // Totals...
    // yes simd      501      529        0
    //  no simd      992     1024        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       42        0
    //    simd2        1        1        0
    //    simd3       10       12        0
    //    simd4       10       14        0
    // Totals...
    // yes simd       45       69        0
    //  no simd       96      136        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircle> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       58        0
    //    simd2        1        1        0
    //    simd3       14       16        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       63       88        0
    //  no simd      128      160        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryCircleRotor> for MultiVector {
    fn mul_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd2        3        5        0
    //    simd3       12       16        0
    //    simd4        9       11        0
    // Totals...
    // yes simd       42       64        0
    //  no simd       96      134        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70       90        0
    //    simd2        3        3        0
    //    simd3       24       28        0
    //    simd4       11       11        0
    // Totals...
    // yes simd      108      132        0
    //  no simd      192      224        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for MultiVector {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd2        1        3        0
    //    simd3        4        8        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       14       37        0
    //  no simd       32       74        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryQuadNum> for MultiVector {
    fn mul_assign(&mut self, other: MysteryQuadNum) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       76        0
    //    simd2        4        4        0
    //    simd3       25       28        0
    //    simd4       22       22        0
    // Totals...
    // yes simd      104      130        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorEven> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       55       78        0
    //    simd2        4        4        0
    //    simd3       27       30        0
    //    simd4       20       20        0
    // Totals...
    // yes simd      106      132        0
    //  no simd      224      256        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       69        0
    //    simd3        7        9        0
    // Totals...
    // yes simd       58       78        0
    //  no simd       72       96        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullCircleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       51        0
    //    simd2        2        3        0
    //    simd3        7        9        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       48       66        0
    //  no simd       73       96        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       32        0
    //    simd2        3        4        0
    //    simd3        7        8        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       49       60        0
    //  no simd      111      128        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       20        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       26        0
    //  no simd        8       39        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullSphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       42        0
    //    simd3       10       11        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       48       67        0
    //  no simd      107      131        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       24        0
    //    simd2        0        2        0
    //    simd3        1        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       32        0
    //  no simd        8       48        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Origin> for MultiVector {
    fn mul_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       55        0
    //    simd2        1        1        0
    //    simd3       12       15        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       50       78        0
    //  no simd       96      130        0
    fn mul(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       60        0
    //    simd3        8       12        0
    // Totals...
    // yes simd       48       72        0
    //  no simd       64       96        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd2        1        1        0
    //    simd3       12       16        0
    //    simd4       10       13        0
    // Totals...
    // yes simd       41       60        0
    //  no simd       96      132        0
    fn mul(self, other: QuadNum) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNum> for MultiVector {
    fn mul_assign(&mut self, other: QuadNum) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       44        0
    //    simd3        8       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       36       60        0
    //  no simd       64       96        0
    fn mul(self, other: QuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNumAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: QuadNumAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       33        0
    //    simd3        5        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       19       45        0
    //  no simd       32       72        0
    fn mul(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNumAligningOriginAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       43        0
    //    simd2        1        1        0
    //    simd3        9       13        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       33       61        0
    //  no simd       64      100        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNumAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: QuadNumAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       35        0
    //    simd3        3        8        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       17       46        0
    //  no simd       32       71        0
    fn mul(self, other: QuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNumOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: QuadNumOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       34        0
    //    simd2        1        1        0
    //    simd3        8       12        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       29       55        0
    //  no simd       64      104        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<QuadNumOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       46        0
    //    simd2        3        3        0
    //    simd3       14       16        0
    //    simd4       13       17        0
    // Totals...
    // yes simd       58       82        0
    //  no simd      128      168        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPoint> for MultiVector {
    fn mul_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd3        4        8        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       12       36        0
    //  no simd       32       76        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<RoundPointAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
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
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       58        0
    //    simd2        2        2        0
    //    simd3       15       18        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       63       89        0
    //  no simd      128      160        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for MultiVector {
    fn mul_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd2        1        2        0
    //    simd3        4        8        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       14       29        0
    //  no simd       32       68        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       36        0
    //    simd2        1        1        0
    //    simd3       10       13        0
    //    simd4       12       14        0
    // Totals...
    // yes simd       42       64        0
    //  no simd       99      133        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      119      151        0
    //    simd2        4        4        0
    //    simd3       55       55        0
    //    simd4       47       47        0
    // Totals...
    // yes simd      225      257        0
    //  no simd      480      512        0
    fn mul(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEven> for MultiVector {
    fn mul_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       99      131        0
    //    simd3       39       39        0
    //    simd4       34       34        0
    // Totals...
    // yes simd      172      204        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAligningOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      103      133        0
    //    simd2        4        4        0
    //    simd3       43       44        0
    //    simd4       28       28        0
    // Totals...
    // yes simd      178      209        0
    //  no simd      352      385        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70       99        0
    //    simd3       26       27        0
    //    simd4       19       19        0
    // Totals...
    // yes simd      115      145        0
    //  no simd      224      256        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenAtOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       74        0
    //    simd3       21       23        0
    //    simd4       28       29        0
    // Totals...
    // yes simd      101      126        0
    //  no simd      227      259        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOnOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      107      139        0
    //    simd2        4        4        0
    //    simd3       39       39        0
    //    simd4       30       30        0
    // Totals...
    // yes simd      180      212        0
    //  no simd      352      384        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      120      152        0
    //    simd2       12       12        0
    //    simd3       52       52        0
    //    simd4       45       45        0
    // Totals...
    // yes simd      229      261        0
    //  no simd      480      512        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for MultiVector {
    fn mul_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       95      124        0
    //    simd2        8        8        0
    //    simd3       43       44        0
    //    simd4       28       28        0
    // Totals...
    // yes simd      174      204        0
    //  no simd      352      384        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       92      122        0
    //    simd2       11       12        0
    //    simd3       38       38        0
    //    simd4       31       31        0
    // Totals...
    // yes simd      172      203        0
    //  no simd      352      384        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn neg(self) -> Self {
        use crate::elements::*;
        let negation = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(-1.0)),
            // e5
            (self[e1] * -1.0),
            // e41, e42, e43, e45
            (self.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (self.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (self.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            (self.group9() * Simd32x4::from(-1.0)),
            // e3215
            (self[e45] * -1.0),
        );
        return negation;
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn not(self) -> Self::Output {
        use crate::elements::*;
        let right_dual = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[1] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group9()[1] * -1.0), (self.group9()[2] * -1.0), (self.group9()[3] * -1.0), self.group9()[0]]),
            // e5
            self[e45],
            // e41, e42, e43, e45
            Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], (self.group6()[3] * -1.0)]),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            Simd32x3::from([self.group6()[0], self.group6()[1], self.group6()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group5()[0] * -1.0), (self.group5()[1] * -1.0), (self.group5()[2] * -1.0), self.group3()[3]]),
            // e423, e431, e412
            (Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group4() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group1()[3] * -1.0), self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e3215
            (self[e1] * -1.0),
        );
        return right_dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       10        0        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group2()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group1()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]])),
            // e5
            (self[e1] - other.group3()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]])),
            // e5
            (self[e1] - other.group3()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - swizzle!(other.group1(), 1, 2, 3, 0)),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - swizzle!(other.group1(), 1, 2, 3, 0)),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group2()[3])]),
            // e5
            (self[e1] - other.group0()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group2()[3])]),
            // e5
            (self[e1] - other.group0()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other[e321])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other[e321])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlector> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlector) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLine> for MultiVector {
    fn sub_assign(&mut self, other: AntiLine) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - other.group0()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        5        0        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[e31]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[e31]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[0])]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryQuadNum> for MultiVector {
    fn sub_assign(&mut self, other: AntiMysteryQuadNum) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[0])]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group0()[3]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlane> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlane) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group0()[3]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNum> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNum) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] - other.group0()[0]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] - other.group0()[0]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[1])]),
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
            (self[e45] - other.group0()[0]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNumAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[2]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[1])]),
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
            (self[e45] - other.group0()[0]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNumOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[1]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[2])]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[0]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[1]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e12345])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Circle> for MultiVector {
    fn sub_assign(&mut self, other: Circle) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group2()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       10        0        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group2()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group1()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - other.group2()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Dipole> for MultiVector {
    fn sub_assign(&mut self, other: Dipole) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - other.group2()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
            // e15, e25, e35
            (self.group4() - other.group1()),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
            // e15, e25, e35
            (self.group4() - other.group1()),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - other.group1()),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - other.group1()),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] - other.group3()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversion) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] - other.group3()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group2()[0]),
                (self.group9()[2] - other.group2()[1]),
                (self.group9()[3] - other.group2()[2]),
            ]),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - other.group1()),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group2()[0]),
                (self.group9()[2] - other.group2()[1]),
                (self.group9()[3] - other.group2()[2]),
            ]),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group1()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
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
            (self.group9() - other.group1()),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
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
            (self.group9() - other.group1()),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       11        0        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - other.group0()),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        9        0        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - other.group2()),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - other.group2()),
            // e23, e31, e12
            (self.group5() - other.group1()),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e45])]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e45])]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPoint> for MultiVector {
    fn sub_assign(&mut self, other: FlatPoint) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - other.group0()),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - other.group0()),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group1()[0]),
                (self.group9()[2] - other.group1()[1]),
                (self.group9()[3] - other.group1()[2]),
            ]),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group1()[0]),
                (self.group9()[2] - other.group1()[1]),
                (self.group9()[3] - other.group1()[2]),
            ]),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            (self[e45] - other.group0()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
            (self[e45] - other.group0()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[0])]),
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[1]),
                (self.group9()[2] - other.group0()[2]),
                (self.group9()[3] - other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[0])]),
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[1]),
                (self.group9()[2] - other.group0()[2]),
                (self.group9()[3] - other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] - other[e3215]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self[e45] - other[e3215]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Infinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other[e5]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Infinity> for MultiVector {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other[e5]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - other.group1()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group8() - other.group0()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group8() - other.group0()),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[3]),
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
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[3]),
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
            (self.group8() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group0()[0]),
                (self.group6()[1] - other.group0()[1]),
                (self.group6()[2] - other.group0()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd2        1        0        0
    //    simd3        4        0        0
    //    simd4        4        0        0
    // Totals...
    // yes simd       11        0        0
    //  no simd       32        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() - other.group0()),
            // e1, e2, e3, e4
            (self.group1() - other.group1()),
            // e5
            (self[e1] - other[e1]),
            // e41, e42, e43, e45
            (self.group3() - other.group3()),
            // e15, e25, e35
            (self.group4() - other.group4()),
            // e23, e31, e12
            (self.group5() - other.group5()),
            // e415, e425, e435, e321
            (self.group6() - other.group6()),
            // e423, e431, e412
            (self.group7() - other.group7()),
            // e235, e315, e125
            (self.group8() - other.group8()),
            // e1234, e4235, e4315, e4125
            (self.group9() - other.group9()),
            // e3215
            (self[e45] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (self.group0() - other.group0()),
            // e1, e2, e3, e4
            (self.group1() - other.group1()),
            // e5
            (self[e1] - other[e1]),
            // e41, e42, e43, e45
            (self.group3() - other.group3()),
            // e15, e25, e35
            (self.group4() - other.group4()),
            // e23, e31, e12
            (self.group5() - other.group5()),
            // e415, e425, e435, e321
            (self.group6() - other.group6()),
            // e423, e431, e412
            (self.group7() - other.group7()),
            // e235, e315, e125
            (self.group8() - other.group8()),
            // e1234, e4235, e4315, e4125
            (self.group9() - other.group9()),
            // e3215
            (self[e45] - other[e45]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryCircle> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircle) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e425])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for MultiVector {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e425])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group0()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipole> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipole) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group1()[0]),
                (self.group9()[2] - other.group1()[1]),
                (self.group9()[3] - other.group1()[2]),
            ]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for MultiVector {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group1()[0]),
                (self.group9()[2] - other.group1()[1]),
                (self.group9()[3] - other.group1()[2]),
            ]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: MysteryQuadNum) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryQuadNum> for MultiVector {
    fn sub_assign(&mut self, other: MysteryQuadNum) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[0])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryVersorEven> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorEven) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryVersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[1]),
                (self.group9()[2] - other.group0()[2]),
                (self.group9()[3] - other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryVersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: MysteryVersorOdd) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[1]),
                (self.group9()[2] - other.group0()[2]),
                (self.group9()[3] - other.group0()[3]),
            ]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - other.group0()),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
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
            Simd32x4::from([(self.group9()[0] - other.group0()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other[e1234]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other[e1234]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e4])]),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e4])]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[0]),
                (self.group9()[2] - other.group0()[1]),
                (self.group9()[3] - other.group0()[2]),
            ]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[0]),
                (self.group9()[2] - other.group0()[1]),
                (self.group9()[3] - other.group0()[2]),
            ]),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[0]),
                (self.group9()[2] - other.group0()[1]),
                (self.group9()[3] - other.group0()[2]),
            ]),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group0()[0]),
                (self.group9()[2] - other.group0()[1]),
                (self.group9()[3] - other.group0()[2]),
            ]),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: QuadNum) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNum> for MultiVector {
    fn sub_assign(&mut self, other: QuadNum) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: QuadNumAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[2])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: QuadNumAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[2])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOriginAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumAligningOriginAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: QuadNumAligningOriginAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: QuadNumAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[2])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[1])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: QuadNumAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[2])]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            (self[e1] - other.group0()[0]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[1])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: QuadNumOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: QuadNumOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[2])]),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            (self[e1] - other[e2]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<RoundPoint> for MultiVector {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group0()),
            // e5
            (self[e1] - other[e2]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e5
            (self[e1] - other.group0()[1]),
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() - Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Sphere> for MultiVector {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() - Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e3215
            (self[e45] - other.group0()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[1]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[0]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            Simd32x4::from([(self.group9()[0] - other.group0()[1]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group0()[0]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() - swizzle!(other.group0(), 3, 0, 1, 2)),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<SphereOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: SphereOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
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
            (self.group9() - swizzle!(other.group0(), 3, 0, 1, 2)),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() - other.group3()),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEven> for MultiVector {
    fn sub_assign(&mut self, other: VersorEven) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            (self.group1() - other.group3()),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[3])]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAligningOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAligningOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[3])]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] - other.group0()[1]),
                (self.group1()[1] - other.group0()[2]),
                (self.group1()[2] - other.group0()[3]),
                self.group1()[3],
            ]),
            // e5
            (self[e1] - other.group2()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            (self.group6() - other.group1()),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        8        0        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenOnOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOnOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (self.group0()[1] - other.group0()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group1()[3])]),
            // e5
            self[e1],
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group6()[0] - other.group1()[0]),
                (self.group6()[1] - other.group1()[1]),
                (self.group6()[2] - other.group1()[2]),
                self.group6()[3],
            ]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group2()),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            self.group0(),
            // e1, e2, e3, e4
            (self.group1() - other.group2()),
            // e5
            (self[e1] - other.group1()[3]),
            // e41, e42, e43, e45
            self.group3(),
            // e15, e25, e35
            self.group4(),
            // e23, e31, e12
            self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], (self.group6()[3] - other.group0()[3])]),
            // e423, e431, e412
            (self.group7() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            (self.group8() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e1234, e4235, e4315, e4125
            self.group9(),
            // e3215
            self[e45],
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] - other.group3()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOdd> for MultiVector {
    fn sub_assign(&mut self, other: VersorOdd) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            (self.group3() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            (self.group9() - Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]])),
            // e3215
            (self[e45] - other.group3()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group2()[0]),
                (self.group9()[2] - other.group2()[1]),
                (self.group9()[3] - other.group2()[2]),
            ]),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOddAtInfinity> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddAtInfinity) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[0]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                self.group9()[0],
                (self.group9()[1] - other.group2()[0]),
                (self.group9()[2] - other.group2()[1]),
                (self.group9()[3] - other.group2()[2]),
            ]),
            // e3215
            (self[e45] - other.group2()[3]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for MultiVector {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[0] - other.group0()[3]), self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e5
            self[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
            // e15, e25, e35
            (self.group4() - Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e23, e31, e12
            (self.group5() - Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e415, e425, e435, e321
            self.group6(),
            // e423, e431, e412
            self.group7(),
            // e235, e315, e125
            self.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(self.group9()[0] - other.group2()[3]), self.group9()[1], self.group9()[2], self.group9()[3]]),
            // e3215
            (self[e45] - other.group1()[3]),
        );
        *self = subtraction;
    }
}
