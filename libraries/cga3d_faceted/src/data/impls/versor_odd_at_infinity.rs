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
// Total Implementations: 507
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       3       0
//  Average:         9      13       0
//  Maximum:       181     211       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        16      21       0
//  Maximum:       352     384       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group2()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group2()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group0()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3]]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiLine) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (Simd32x4::from([other.group0()[3], other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiMotor) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (Simd32x4::from([other.group0()[3], other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] + self.group2()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[e31]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[e31]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryQuadNum> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryQuadNum> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiMysteryQuadNum) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlane> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNum> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[2] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiQuadNumAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[2] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group0()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group1()[3]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiVersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
                other.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group0()[1]),
                (other.group1()[1] + self.group0()[2]),
                (other.group1()[2] + self.group0()[3]),
                other.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group1()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] + self.group2()[0]),
                (other.group1()[2] + self.group2()[1]),
                (other.group1()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e45])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatOrigin> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e45])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPoint> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPoint> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: FlatPoint) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Flector> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: Flector) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (other.group0()[0] + self.group0()[1]),
                (other.group0()[1] + self.group0()[2]),
                (other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group2()[0]),
                (other.group0()[2] + self.group2()[1]),
                (other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group2()[0]),
                (other.group0()[2] + self.group2()[1]),
                (other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Horizon> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e3215])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Horizon> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e3215])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Infinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Line> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       12        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] + self.group0()[0]), other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e41, e42, e43, e45
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (other.group3()[3] + self.group1()[3])]),
            // e15, e25, e35
            (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) + other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                other.group9()[0],
                (other.group9()[1] + self.group2()[0]),
                (other.group9()[2] + self.group2()[1]),
                (other.group9()[3] + self.group2()[2]),
            ]),
            // e3215
            (self.group2()[3] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircle> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other[e425]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipole> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipole> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: MysteryDipole) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[0] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group2()[0]),
                (other.group0()[2] + self.group2()[1]),
                (other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryVersorOdd> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: MysteryVersorOdd) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[0] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group2()[0]),
                (other.group0()[2] + self.group2()[1]),
                (other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryVersorRoundPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorRoundPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorSphere> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorSphere) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryVersorSphere> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: MysteryVersorSphere) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[3]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other[e1234]]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Plane> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: Plane) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNum> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: QuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: QuadNumAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[2]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[scalar]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[scalar]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other[e4315]]),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[1]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group0()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group0()[1]),
                (other.group2()[1] + self.group0()[2]),
                (other.group2()[2] + self.group0()[3]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOddAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: VersorOddAtInfinity) {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[0] + other.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[1] + other.group2()[0]),
                (self.group0()[2] + other.group2()[1]),
                (self.group0()[3] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorRoundPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorRoundPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group1()[1]]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other.group1()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorRoundPointAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[2]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorRoundPointAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group1()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group1()[0],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorSphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: VersorSphere) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[1] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group1()[0]]),
            // e4235, e4315, e4125, e3215
            (self.group2() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::Add<VersorSphereAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[e4315]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (self.group2() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorSphereAtInfinity> for VersorOddAtInfinity {
    fn add_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] + other[e4315]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (self.group2() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorSphereOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], other.group0()[1]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       16        0
    //  no simd       10       21        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       27       43        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       23       39        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       19       31        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       17       33        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       31       47        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       33        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       22       34        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for VersorOddAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3        8        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for VersorOddAtInfinity {
    type Output = MysteryQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorOddAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       30        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       23        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       15        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       16       28        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
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
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       13       25        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       18       26        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        4       16        0
    fn bitxor(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryQuadNum> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       25        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       21        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       18        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        5       17        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for VersorOddAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       11       26        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       18       34        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       13        0
    fn bitxor(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bitxor(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorOddAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for VersorOddAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for VersorOddAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for VersorOddAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for VersorOddAtInfinity {
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
impl std::ops::BitXor<CircleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       11        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorOddAtInfinity {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       18        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       16        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        4       12        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for VersorOddAtInfinity {
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
impl std::ops::BitXor<Dipole> for VersorOddAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       16       31        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       10       22        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       14        0
    //  no simd        8       19        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       18        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorOddAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       25        0
    //  no simd       21       36        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       15       27        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       23        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       15       29        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       13       27        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for VersorOddAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for VersorOddAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorOddAtInfinity {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for VersorOddAtInfinity {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for VersorOddAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorOddAtInfinity {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for VersorOddAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for VersorOddAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorOddAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for VersorOddAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       52        0
    //    simd3        5        8        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       42       65        0
    //  no simd       64       96        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for VersorOddAtInfinity {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for VersorOddAtInfinity {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4        9        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       13        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd        8       16        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for VersorOddAtInfinity {
    type Output = MysteryQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       19       27        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       16       28        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorRoundPoint> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       22        0
    fn bitxor(self, other: MysteryVersorRoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorSphere> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       15        0
    fn bitxor(self, other: MysteryVersorSphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorSphere> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: MysteryVersorSphere) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for VersorOddAtInfinity {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       13        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for VersorOddAtInfinity {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for VersorOddAtInfinity {
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
impl std::ops::BitXor<Origin> for VersorOddAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for VersorOddAtInfinity {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for VersorOddAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       15        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        7        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       14       30        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorOddAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for VersorOddAtInfinity {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for VersorOddAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       28        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       35       48        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       19       28        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       34        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        9       21        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for VersorOddAtInfinity {
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
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       41        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       32       48        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       23       35        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       25       41        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPoint> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       15       31        0
    fn bitxor(self, other: VersorRoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       13        0
    fn bitxor(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       26        0
    fn bitxor(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        1        5        0
    //  no simd        4       17        0
    fn bitxor(self, other: VersorSphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphereAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        4        0
    // no simd        4       16        0
    fn bitxor(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorSphereAtInfinity> for VersorOddAtInfinity {
    fn bitxor_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphereOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn bitxor(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[scalar],
                anti_circle_rotor_aligning_origin_at_infinity[e15],
                anti_circle_rotor_aligning_origin_at_infinity[e25],
                anti_circle_rotor_aligning_origin_at_infinity[e35],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e23],
                anti_circle_rotor_aligning_origin_at_infinity[e31],
                anti_circle_rotor_aligning_origin_at_infinity[e12],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                anti_circle_rotor_at_infinity[scalar],
                anti_circle_rotor_at_infinity[e15],
                anti_circle_rotor_at_infinity[e25],
                anti_circle_rotor_at_infinity[e35],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_at_infinity[e23],
                anti_circle_rotor_at_infinity[e31],
                anti_circle_rotor_at_infinity[e12],
                anti_circle_rotor_at_infinity[e45],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLine> for VersorOddAtInfinity {
    fn from(anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, anti_line[e15], anti_line[e25], anti_line[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_line[e23], anti_line[e31], anti_line[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for VersorOddAtInfinity {
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotor> for VersorOddAtInfinity {
    fn from(anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_motor[scalar], anti_motor[e15], anti_motor[e25], anti_motor[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
        );
    }
}

impl From<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn from(anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_motor_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_mystery_circle_rotor[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_mystery_circle_rotor[e23],
                anti_mystery_circle_rotor[e31],
                anti_mystery_circle_rotor[e12],
                anti_mystery_circle_rotor[e45],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMysteryQuadNum> for VersorOddAtInfinity {
    fn from(anti_mystery_quad_num: AntiMysteryQuadNum) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_mystery_quad_num[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_quad_num[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    fn from(anti_quad_num_at_infinity: AntiQuadNumAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_quad_num_at_infinity[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[e3215]]),
        );
    }
}

impl From<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn from(anti_versor_round_point_aligning_origin_at_infinity: AntiVersorRoundPointAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_versor_round_point_aligning_origin_at_infinity[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_round_point_aligning_origin_at_infinity[e3215]]),
        );
    }
}

impl From<DipoleAtInfinity> for VersorOddAtInfinity {
    fn from(dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12], dipole_at_infinity[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    fn from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion_at_infinity[e15], dipole_inversion_at_infinity[e25], dipole_inversion_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                dipole_inversion_at_infinity[e23],
                dipole_inversion_at_infinity[e31],
                dipole_inversion_at_infinity[e12],
                dipole_inversion_at_infinity[e45],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                dipole_inversion_at_infinity[e4235],
                dipole_inversion_at_infinity[e4315],
                dipole_inversion_at_infinity[e4125],
                dipole_inversion_at_infinity[e3215],
            ]),
        );
    }
}

impl From<FlatOrigin> for VersorOddAtInfinity {
    fn from(flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for VersorOddAtInfinity {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, flat_point[e15], flat_point[e25], flat_point[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for VersorOddAtInfinity {
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for VersorOddAtInfinity {
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, flector[e15], flector[e25], flector[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
        );
    }
}

impl From<FlectorAtInfinity> for VersorOddAtInfinity {
    fn from(flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector_at_infinity[e3215]]),
        );
    }
}

impl From<FlectorOnOrigin> for VersorOddAtInfinity {
    fn from(flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector_on_origin[e4235], flector_on_origin[e4315], flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for VersorOddAtInfinity {
    fn from(horizon: Horizon) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, horizon[e3215]]),
        );
    }
}

impl From<MysteryDipole> for VersorOddAtInfinity {
    fn from(mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([mystery_dipole[e23], mystery_dipole[e31], mystery_dipole[e12], mystery_dipole[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryDipoleInversion> for VersorOddAtInfinity {
    fn from(mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                mystery_dipole_inversion[e23],
                mystery_dipole_inversion[e31],
                mystery_dipole_inversion[e12],
                mystery_dipole_inversion[e45],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_dipole_inversion[e4235], mystery_dipole_inversion[e4315], mystery_dipole_inversion[e4125], 0.0]),
        );
    }
}

impl From<MysteryVersorOdd> for VersorOddAtInfinity {
    fn from(mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([mystery_versor_odd[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([mystery_versor_odd[e23], mystery_versor_odd[e31], mystery_versor_odd[e12], mystery_versor_odd[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        );
    }
}

impl From<MysteryVersorSphere> for VersorOddAtInfinity {
    fn from(mystery_versor_sphere: MysteryVersorSphere) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([mystery_versor_sphere[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_sphere[e4235], mystery_versor_sphere[e4315], mystery_versor_sphere[e4125], 0.0]),
        );
    }
}

impl From<Plane> for VersorOddAtInfinity {
    fn from(plane: Plane) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane[e4235], plane[e4315], plane[e4125], plane[e3215]]),
        );
    }
}

impl From<PlaneOnOrigin> for VersorOddAtInfinity {
    fn from(plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane_on_origin[e4235], plane_on_origin[e4315], plane_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Scalar> for VersorOddAtInfinity {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([scalar[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<VersorSphereAtInfinity> for VersorOddAtInfinity {
    fn from(versor_sphere_at_infinity: VersorSphereAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([versor_sphere_at_infinity[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                versor_sphere_at_infinity[e4235],
                versor_sphere_at_infinity[e4315],
                versor_sphere_at_infinity[e4125],
                versor_sphere_at_infinity[e3215],
            ]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       68       84        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      116      132        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      104      120        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       45       57        0
    //  no simd       60       72        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       44        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       43       54        0
    //  no simd       73       84        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       80       96        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       25       25        0
    // Totals...
    // yes simd       89      105        0
    //  no simd      164      180        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4       19       23        0
    // Totals...
    // yes simd       47       55        0
    //  no simd      104      124        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       36       52        0
    //  no simd       96      112        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       63        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       62       81        0
    //  no simd      116      135        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       20       31        0
    //  no simd       47       61        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       15       23        0
    //  no simd       24       44        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       35       47        0
    //  no simd       68       80        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
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
impl std::ops::Mul<AntiLine> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       48       60        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       53        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       47       60        0
    //  no simd       68       81        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
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
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       48       64        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       72       84        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        8        0
    // no simd       12       32        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryQuadNum> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        7       11        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       32       52        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       36        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       36       52        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        5        6        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       24       32        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       23       41        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       19       37        0
    //  no simd       37       55        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       48       65        0
    //  no simd       96      113        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       20        0
    fn mul(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       20        0
    //  no simd       12       32        0
    fn mul(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      104      120        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       92      108        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       60       72        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       57       72        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       68       84        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       37        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       35       49        0
    //  no simd       71       85        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      116      132        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      104      120        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       60       72        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       72       84        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       35       51        0
    //  no simd       80       96        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      104      120        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       44        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       69       84        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       44        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       40       51        0
    //  no simd       61       72        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       57       72        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       25       25        0
    // Totals...
    // yes simd       89      105        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       59       75        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       56        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       60       71        0
    //  no simd      105      116        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       39       54        0
    //  no simd       81       96        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       47        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       46       64        0
    //  no simd       97      115        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       40        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       27       46        0
    //  no simd       45       64        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       92      108        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        5        0
    // no simd        0       20        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       32        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       24       33        0
    //  no simd       24       36        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       18        0
    //  no simd       20       24        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       42       50        0
    //  no simd       72       80        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       20        0
    //  no simd       28       32        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       36       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorOddAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        2        0
    // no simd        4        8        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       48       60        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       18        0
    //  no simd       20       24        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       36        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       24        0
    //    simd4       12       14        0
    // Totals...
    // yes simd       32       38        0
    //  no simd       68       80        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorOddAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       20        0
    //  no simd       28       32        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
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
impl std::ops::Mul<MultiVector> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      134        0
    //    simd2        8        8        0
    //    simd3       41       42        0
    //    simd4       27       27        0
    // Totals...
    // yes simd      181      211        0
    //  no simd      352      384        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       48       60        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       36       52        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       45       57        0
    //  no simd       72       84        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       12       24        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       36       48        0
    //  no simd       84       96        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       84       96        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorRoundPoint> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       36       48        0
    fn mul(self, other: MysteryVersorRoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorSphere> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        8       12        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       36       56        0
    fn mul(self, other: MysteryVersorSphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorSphere> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: MysteryVersorSphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       37       48        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       37       48        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       40        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       35       46        0
    //  no simd       53       64        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        4       16        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       23       28        0
    //  no simd       56       64        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       17        0
    //  no simd        4       23        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       33        0
    //  no simd       32       48        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        9       12        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       36       55        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       20       32        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        5        7        0
    // Totals...
    // yes simd        5       15        0
    //  no simd       20       36        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       31        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       21       39        0
    //  no simd       45       63        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        5       19        0
    //  no simd        8       31        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       23       40        0
    //  no simd       47       64        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       21        0
    //  no simd        8       24        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       22        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       41       58        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       64        0
    //    simd4       31       32        0
    // Totals...
    // yes simd       83       96        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       25       26        0
    // Totals...
    // yes simd       53       66        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       56       68        0
    //  no simd      116      128        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       32        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       39       48        0
    //  no simd       84       96        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       21       22        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       96      112        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       56       72        0
    //  no simd      128      144        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       63       74        0
    //  no simd      117      128        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      128      144        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPoint> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       39        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       30       48        0
    //  no simd       57       75        0
    fn mul(self, other: VersorRoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       20       44        0
    fn mul(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        7        0
    // no simd        8       28        0
    fn mul(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAtInfinity> for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       44       56        0
    fn mul(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        2        5        0
    // Totals...
    // yes simd        6       21        0
    //  no simd       12       36        0
    fn mul(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       36        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       29       46        0
    //  no simd       59       76        0
    fn mul(self, other: VersorSphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphereAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       44       60        0
    fn mul(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorSphereAtInfinity> for VersorOddAtInfinity {
    fn mul_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphereOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       13        0
    //    simd4        5        7        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       23       41        0
    fn mul(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorOddAtInfinity {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn neg(self) -> Self {
        let negation = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (self.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (self.group2() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for VersorOddAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group2()[3] + self.group0()[0]),
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        3        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group2()[3] + self.group0()[0]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-swizzle!(other.group1(), 3, 0, 1, 2) + self.group0()),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group0()[3] + self.group0()[0]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOddAtInfinity {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorOddAtInfinity {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group2()[3] * -1.0)]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiLine) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-Simd32x4::from([other.group0()[3], other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[3] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiMotor) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-Simd32x4::from([other.group0()[3], other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[3] + self.group2()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[e31]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryCircleRotor> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[e31]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryQuadNum> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryQuadNum> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiMysteryQuadNum) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group0()[3] + self.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[2] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiQuadNumAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[2] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group0()[3] + self.group0()[0]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group1()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[1] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiVersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group0()[1] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        7        3        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        3        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd       11        4        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
                (other.group1()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group0()[1]),
                (-other.group1()[1] + self.group0()[2]),
                (-other.group1()[2] + self.group0()[3]),
                (other.group1()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group1()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[1] + self.group2()[0]),
                (-other.group1()[2] + self.group2()[1]),
                (-other.group1()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        4        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e45])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatOrigin> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e45])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPoint> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: FlatPoint) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            (-other.group1() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Flector> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: Flector) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            (-other.group1() + self.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                self.group0()[0],
                (-other.group0()[0] + self.group0()[1]),
                (-other.group0()[1] + self.group0()[2]),
                (-other.group0()[2] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group2()[0]),
                (-other.group0()[2] + self.group2()[1]),
                (-other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group2()[0]),
                (-other.group0()[2] + self.group2()[1]),
                (-other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Horizon> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e3215])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Horizon> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e3215])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Infinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other[e5] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        6        0
    //    simd3        2        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       12       20        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(-other.group0()[0] + self.group0()[0]), (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (-other.group3()[3] + self.group1()[3]),
            ]),
            // e15, e25, e35
            (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]) - other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([
                (other.group9()[0] * -1.0),
                (-other.group9()[1] + self.group2()[0]),
                (-other.group9()[2] + self.group2()[1]),
                (-other.group9()[3] + self.group2()[2]),
            ]),
            // e3215
            (self.group2()[3] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other[e425] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipole> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: MysteryDipole) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: MysteryQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[0] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group2()[0]),
                (-other.group0()[2] + self.group2()[1]),
                (-other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryVersorOdd> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: MysteryVersorOdd) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[0] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group2()[0]),
                (-other.group0()[2] + self.group2()[1]),
                (-other.group0()[3] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryVersorRoundPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MysteryVersorRoundPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorSphere> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MysteryVersorSphere) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryVersorSphere> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: MysteryVersorSphere) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(-other.group0()[3] + self.group0()[0]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other[e1234] * -1.0)]),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Plane> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: Plane) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNum> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: QuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: QuadNumAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[2] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for VersorOddAtInfinity {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[scalar]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[scalar]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other[e4315] * -1.0)]),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[1] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorOddAtInfinity {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], 0.0]),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        4        0
    //  no simd       12        4        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group0()[3] + self.group0()[0]),
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group0()[1]),
                (-other.group2()[1] + self.group0()[2]),
                (-other.group2()[2] + self.group0()[3]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        0        0
    // no simd       12        0        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOddAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: VersorOddAtInfinity) {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        4        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[0] - other.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[1] - other.group2()[0]),
                (self.group0()[2] - other.group2()[1]),
                (self.group0()[3] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorRoundPoint> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn sub(self, other: VersorRoundPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group1()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other.group1()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAligningOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[2] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAligningOriginAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAtInfinity> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn sub(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group1()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[0], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e3215
            self.group2()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorSphere> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        5        1        0
    fn sub(self, other: VersorSphere) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group1()[1] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group1()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            (self.group2() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorSphereAtInfinity> for VersorOddAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[e4315]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (self.group2() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorSphereAtInfinity> for VersorOddAtInfinity {
    fn sub_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([(self.group0()[0] - other[e4315]), self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e4235, e4315, e4125, e3215
            (self.group2() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorSphereOrthogonalOrigin> for VersorOddAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        1        0
    fn sub(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group0()[2] + self.group0()[0])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[3], (other.group0()[1] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleOnOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from AntiCircleOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotor> for VersorOddAtInfinity {
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
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_circle_rotor[scalar], anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12], anti_circle_rotor[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for VersorOddAtInfinity {
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
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[scalar],
                anti_circle_rotor_aligning_origin[e15],
                anti_circle_rotor_aligning_origin[e25],
                anti_circle_rotor_aligning_origin[e35],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e23],
                anti_circle_rotor_aligning_origin[e31],
                anti_circle_rotor_aligning_origin[e12],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for VersorOddAtInfinity {
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
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_circle_rotor_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor_on_origin[e23], anti_circle_rotor_on_origin[e31], anti_circle_rotor_on_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNum> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_quad_num: AntiQuadNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNum do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_quad_num[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e3215]]),
        ));
    }
}

impl TryFrom<AntiQuadNumOrthogonalOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_quad_num_orthogonal_origin: AntiQuadNumOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNumOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e3215]]),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for VersorOddAtInfinity {
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
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_versor_even_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_versor_even_on_origin[e23], anti_versor_even_on_origin[e31], anti_versor_even_on_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorRoundPointOnOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(anti_versor_round_point_on_origin: AntiVersorRoundPointOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_round_point_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorRoundPointOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([anti_versor_round_point_on_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for VersorOddAtInfinity {
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
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole[e15], dipole[e25], dipole[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], dipole[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_aligning_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for VersorOddAtInfinity {
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
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], dipole_inversion[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion[e4235], dipole_inversion[e4315], dipole_inversion[e4125], dipole_inversion[e3215]]),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion_aligning_origin[e15], dipole_inversion_aligning_origin[e25], dipole_inversion_aligning_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_aligning_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                dipole_inversion_aligning_origin[e4235],
                dipole_inversion_aligning_origin[e4315],
                dipole_inversion_aligning_origin[e4125],
                dipole_inversion_aligning_origin[e3215],
            ]),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_inversion_at_origin[e15], dipole_inversion_at_origin[e25], dipole_inversion_at_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_origin[e3215]]),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion_on_origin[e4235], dipole_inversion_on_origin[e4315], dipole_inversion_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                0.0,
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e23],
                dipole_inversion_orthogonal_origin[e31],
                dipole_inversion_orthogonal_origin[e12],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_orthogonal_origin[e3215]]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_on_origin[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorOddAtInfinity {
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
            let mut error = "Elements from MultiVector do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([multi_vector[scalar], multi_vector[e15], multi_vector[e25], multi_vector[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<Sphere> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            let mut error = "Elements from Sphere do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere[e4235], sphere[e4315], sphere[e4125], sphere[e3215]]),
        ));
    }
}

impl TryFrom<SphereAtOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from SphereAtOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e3215]]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for VersorOddAtInfinity {
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
            let mut error = "Elements from SphereOnOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125], 0.0]),
        ));
    }
}

impl TryFrom<VersorOdd> for VersorOddAtInfinity {
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
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([versor_odd[scalar], versor_odd[e15], versor_odd[e25], versor_odd[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e45]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd[e4235], versor_odd[e4315], versor_odd[e4125], versor_odd[e3215]]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for VersorOddAtInfinity {
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
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([
                versor_odd_orthogonal_origin[scalar],
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e23], versor_odd_orthogonal_origin[e31], versor_odd_orthogonal_origin[e12], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_orthogonal_origin[e3215]]),
        ));
    }
}

impl TryFrom<VersorSphere> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(versor_sphere: VersorSphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_sphere[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorSphere do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([versor_sphere[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_sphere[e4235], versor_sphere[e4315], versor_sphere[e4125], versor_sphere[e3215]]),
        ));
    }
}

impl TryFrom<VersorSphereOrthogonalOrigin> for VersorOddAtInfinity {
    type Error = String;
    fn try_from(versor_sphere_orthogonal_origin: VersorSphereOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_sphere_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorSphereOrthogonalOrigin do not fit into VersorOddAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([versor_sphere_orthogonal_origin[scalar], 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_sphere_orthogonal_origin[e3215]]),
        ));
    }
}
