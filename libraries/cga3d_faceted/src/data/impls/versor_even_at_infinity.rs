use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 387
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       3       0
//  Average:        10      14       0
//  Maximum:       178     205       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       0       0
//   Median:         4       4       0
//  Average:        18      23       0
//  Maximum:       356     385       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group3()[0]),
                (self.group0()[2] + other.group3()[1]),
                (self.group0()[3] + other.group3()[2]),
                other.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group2()[0]),
                (self.group0()[2] + other.group2()[1]),
                (self.group0()[3] + other.group2()[2]),
            ]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group2()[0]),
                (self.group0()[2] + other.group2()[1]),
                (self.group0()[3] + other.group2()[2]),
            ]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]])),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                (self.group0()[3] + other.group1()[3]),
                other.group1()[0],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group2()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlector> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group1()[0]),
                (self.group0()[2] + other.group1()[1]),
                (self.group0()[3] + other.group1()[2]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlector> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiFlector) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group1()[0]),
                (self.group0()[2] + other.group1()[1]),
                (self.group0()[3] + other.group1()[2]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                (self.group0()[3] + other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                (self.group0()[3] + other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLine> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotor> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlane> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiPlane) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiScalar> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other[e12345]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other[e12345]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
                other.group0()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: Circle) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group2()[3])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group2()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[1]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[0])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DualNum> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: DualNum) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[1]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[0])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Infinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: Line) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: LineAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineOnOrigin> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: LineOnOrigin) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Motor> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: Motor) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] + other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MultiVector> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       12        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], (self.group0()[0] + other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group1()[0]),
                (self.group0()[2] + other.group1()[1]),
                (self.group0()[3] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e5
            (self.group2()[3] + other[e1]),
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            (self.group1() + other.group6()),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            (other.group8() + Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]])),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other[e4]]),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e2])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group0()[0]),
                (self.group0()[2] + other.group0()[1]),
                (self.group0()[3] + other.group0()[2]),
                other.group0()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[1], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group3()[0]),
                (self.group0()[2] + other.group3()[1]),
                (self.group0()[3] + other.group3()[2]),
                other.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] + other.group2()[0]),
                (self.group0()[2] + other.group2()[1]),
                (self.group0()[3] + other.group2()[2]),
                other.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       10       21        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       37        0
    //  no simd       28       46        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       36        0
    //  no simd       24       42        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       22       31        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       21       33        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       27        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       31       53        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       22       32        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       26        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       30        0
    //  no simd       18       40        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       31        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for VersorEvenAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorEvenAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       16       29        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       22        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorEvenAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       21        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       12       25        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       21        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd3        0        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        8       21        0
    //  no simd       11       34        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       22       34        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       10       15        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        6       10        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for VersorEvenAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       13        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd       10       15        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        6       10        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for VersorEvenAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       20       31        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       12       22        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for VersorEvenAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       22        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        8       18        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorEvenAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       24       38        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       19        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       16       29        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       11       22        0
    //  no simd       14       25        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        9       22        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       17       28        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       16       27        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for VersorEvenAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       14        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for VersorEvenAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorEvenAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        9       16        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for VersorEvenAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       10        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorEvenAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for VersorEvenAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for VersorEvenAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorEvenAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       13        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for VersorEvenAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       55        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       43       67        0
    //  no simd       63       97        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for VersorEvenAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       13        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        6       21        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for VersorEvenAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       28        0
    //  no simd       15       35        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for VersorEvenAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       24        0
    //    simd3        0        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       34       50        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       15       31        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       18        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       22       32        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        9       19        0
    //  no simd        9       25        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       22        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       44        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       23       35        0
    //  no simd       35       47        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       34        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for VersorEvenAtInfinity {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       28       40        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    fn from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_dipole_inversion_at_infinity[e1], anti_dipole_inversion_at_infinity[e2], anti_dipole_inversion_at_infinity[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
                anti_dipole_inversion_at_infinity[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
                anti_dipole_inversion_at_infinity[e5],
            ]),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEvenAtInfinity {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEvenAtInfinity {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        );
    }
}

impl From<AntiFlector> for VersorEvenAtInfinity {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_flector[e1], anti_flector[e2], anti_flector[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiPlane> for VersorEvenAtInfinity {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_plane[e1], anti_plane[e2], anti_plane[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiScalar> for VersorEvenAtInfinity {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_scalar[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for VersorEvenAtInfinity {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    fn from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([circle_rotor_aligning_origin_at_infinity[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e415],
                circle_rotor_aligning_origin_at_infinity[e425],
                circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e235],
                circle_rotor_aligning_origin_at_infinity[e315],
                circle_rotor_aligning_origin_at_infinity[e125],
                0.0,
            ]),
        );
    }
}

impl From<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    fn from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([circle_rotor_at_infinity[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor_at_infinity[e235], circle_rotor_at_infinity[e315], circle_rotor_at_infinity[e125], 0.0]),
        );
    }
}

impl From<DualNum> for VersorEvenAtInfinity {
    fn from(dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([dual_num[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
        );
    }
}

impl From<Infinity> for VersorEvenAtInfinity {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
        );
    }
}

impl From<Line> for VersorEvenAtInfinity {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
        );
    }
}

impl From<LineAtInfinity> for VersorEvenAtInfinity {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
        );
    }
}

impl From<LineOnOrigin> for VersorEvenAtInfinity {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEvenAtInfinity {
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([motor[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e5]]),
        );
    }
}

impl From<MotorAtInfinity> for VersorEvenAtInfinity {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
        );
    }
}

impl From<MotorOnOrigin> for VersorEvenAtInfinity {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([motor_on_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       68       84        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      116      132        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      104      120        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       60       72        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       72       84        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       80       96        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       71        0
    //    simd4       28       28        0
    // Totals...
    // yes simd       80       99        0
    //  no simd      164      183        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4       20       22        0
    // Totals...
    // yes simd       48       54        0
    //  no simd      108      120        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       49       64        0
    //  no simd       97      112        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      116      132        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       48        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       32       52        0
    //  no simd       44       64        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        5        0
    // no simd        8       20        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       24       35        0
    //  no simd       24       41        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       40        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       39       50        0
    //  no simd       69       80        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       48       60        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       68       80        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       35       48        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       37       59        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       96      112        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       59       75        0
    //  no simd      104      120        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       92      108        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       31       36        0
    //  no simd       64       72        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       44       60        0
    //  no simd       56       72        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       68       84        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       68       84        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      116      132        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      104      120        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       60       72        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4       14       15        0
    // Totals...
    // yes simd       34       43        0
    //  no simd       76       88        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       80       96        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      104      120        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       14       16        0
    // Totals...
    // yes simd       26       40        0
    //  no simd       68       88        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       60       72        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       35       49        0
    //  no simd       56       76        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       34       34        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       47       63        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       35       47        0
    //  no simd      104      116        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       35       49        0
    //  no simd       80       97        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       96      112        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       56       72        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        8       12        0
    // Totals...
    // yes simd       20       32        0
    //  no simd       44       68        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       92      108        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       19        0
    //  no simd        8       25        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6       11        0
    // no simd       24       44        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        5        9        0
    //  no simd       20       24        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       68       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        7        8        0
    // no simd       28       32        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorEvenAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       48       60        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        5        9        0
    //  no simd       20       24        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       39       44        0
    //  no simd       72       80        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorEvenAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        7        8        0
    // no simd       28       32        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      103      129        0
    //    simd2        4        4        0
    //    simd3       39       40        0
    //    simd4       32       32        0
    // Totals...
    // yes simd      178      205        0
    //  no simd      356      385        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       36       52        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       55       65        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       17        0
    //  no simd        4       23        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       37       49        0
    //  no simd       52       64        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       18        0
    //  no simd        4       24        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       44        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       27        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       21       37        0
    //  no simd       45       67        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        5       21        0
    //  no simd        8       36        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       44       60        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       19        0
    //  no simd        8       31        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       18       29        0
    //  no simd       39       56        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       30       30        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4       23       24        0
    // Totals...
    // yes simd       51       56        0
    //  no simd      120      128        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       49       64        0
    //  no simd       97      112        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd4       39       39        0
    // Totals...
    // yes simd       59       76        0
    //  no simd      176      193        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       35       47        0
    //  no simd      116      128        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for VersorEvenAtInfinity {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       25       25        0
    // Totals...
    // yes simd       53       69        0
    //  no simd      128      144        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Not for VersorEvenAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn not(self) -> Self::Output {
        let dual = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (self.group2()[3] * -1.0)]),
        );
        return dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       11        4        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group3()[0]),
                (self.group0()[2] - other.group3()[1]),
                (self.group0()[3] - other.group3()[2]),
                (other.group2()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group2()[0]),
                (self.group0()[2] - other.group2()[1]),
                (self.group0()[3] - other.group2()[2]),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group2()[0]),
                (self.group0()[2] - other.group2()[1]),
                (self.group0()[3] - other.group2()[2]),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]])),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (self.group0()[3] - other.group1()[3]),
                (other.group1()[0] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        7        4        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group2()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group1()[0]),
                (self.group0()[2] - other.group1()[1]),
                (self.group0()[3] - other.group1()[2]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlector> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiFlector) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group1()[0]),
                (self.group0()[2] - other.group1()[1]),
                (self.group0()[3] - other.group1()[2]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (self.group0()[3] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (self.group0()[3] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLine> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlane> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiPlane) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                self.group0()[0],
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other[e12345]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other[e12345]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        7        3        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        3        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        3        0
    //  no simd        8        3        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        3        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group1()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (other.group1() * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[1]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[0])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DualNum> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: DualNum) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[1]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[0])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other[e3215] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Infinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: Line) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineOnOrigin> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Motor> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: Motor) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(self.group0()[0] - other.group0()[3]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MultiVector> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7        8        0
    //  no simd       12       20        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), (self.group0()[0] - other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group1()[0]),
                (self.group0()[2] - other.group1()[1]),
                (self.group0()[3] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e5
            (self.group2()[3] - other[e1]),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() - other.group6()),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (-other.group8() + Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]])),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other[e1234] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other[e4] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        1        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e2])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group0()[0]),
                (self.group0()[2] - other.group0()[1]),
                (self.group0()[3] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for VersorEvenAtInfinity {
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
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[1] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorEvenAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group3()[0]),
                (self.group0()[2] - other.group3()[1]),
                (self.group0()[3] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for VersorEvenAtInfinity {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        4        4        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] - other.group2()[0]),
                (self.group0()[2] - other.group2()[1]),
                (self.group0()[3] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[0] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorEvenAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}

impl TryFrom<AntiDipoleInversion> for VersorEvenAtInfinity {
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
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435], anti_dipole_inversion[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], anti_dipole_inversion[e5]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
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
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_dipole_inversion_on_origin[e1], anti_dipole_inversion_on_origin[e2], anti_dipole_inversion_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for VersorEvenAtInfinity {
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
        let el = anti_dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e415],
                anti_dipole_inversion_orthogonal_origin[e425],
                anti_dipole_inversion_orthogonal_origin[e435],
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                anti_dipole_inversion_orthogonal_origin[e5],
            ]),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Circle> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotor> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from CircleRotor do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([circle_rotor[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([circle_rotor_aligning_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor_aligning_origin[e235], circle_rotor_aligning_origin[e315], circle_rotor_aligning_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([circle_rotor_on_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorEvenAtInfinity {
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
            let mut error = "Elements from MultiVector do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([multi_vector[e12345], multi_vector[e1], multi_vector[e2], multi_vector[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<RoundPoint> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, round_point[e1], round_point[e2], round_point[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for VersorEvenAtInfinity {
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
            let mut error = "Elements from RoundPointAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEven> for VersorEvenAtInfinity {
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
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even[e12345], versor_even[e1], versor_even[e2], versor_even[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
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
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even_aligning_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e5],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for VersorEvenAtInfinity {
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
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even_on_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
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
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, versor_even_orthogonal_origin[e1], versor_even_orthogonal_origin[e2], versor_even_orthogonal_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e5],
            ]),
        ));
    }
}
