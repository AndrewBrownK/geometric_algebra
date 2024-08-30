use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 483
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       2       0
//  Average:        13      17       0
//  Maximum:       226     258       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       4       0
//  Average:        23      28       0
//  Maximum:       480     512       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3]]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLine> for VersorOdd {
    fn add_assign(&mut self, other: AntiLine) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group1()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiMotor) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group1()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e31])]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryCircleRotor> for VersorOdd {
    fn add_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e31])]),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMysteryQuadNum> for VersorOdd {
    fn add_assign(&mut self, other: AntiMysteryQuadNum) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNum> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNum) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAligningOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNumAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNumOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiVersorEvenOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Circle> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Dipole> for VersorOdd {
    fn add_assign(&mut self, other: Dipole) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAligningOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleAligningOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: DipoleAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleAtOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversion> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversion) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAligningOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversionAligningOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            (other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversionAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group2() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversionAtOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] + self.group3()[0]),
                (other.group1()[2] + self.group3()[1]),
                (other.group1()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversionOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] + self.group3()[0]),
                (other.group1()[2] + self.group3()[1]),
                (other.group1()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       11        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleInversionOrthogonalOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DipoleOrthogonalOrigin> for VersorOdd {
    fn add_assign(&mut self, other: DipoleOrthogonalOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e45])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatOrigin> for VersorOdd {
    fn add_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e45])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPoint> for VersorOdd {
    fn add_assign(&mut self, other: FlatPoint) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Flector> for VersorOdd {
    fn add_assign(&mut self, other: Flector) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (other.group1() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: FlectorAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group3()[0]),
                (other.group0()[2] + self.group3()[1]),
                (other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<FlectorOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: FlectorOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group3()[0]),
                (other.group0()[2] + self.group3()[1]),
                (other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Horizon> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e3215])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Horizon> for VersorOdd {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e3215])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Infinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Line> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       16        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] + self.group0()[3]), other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e41, e42, e43, e45
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) + other.group3()),
            // e15, e25, e35
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) + other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]) + other.group9()),
            // e3215
            (self.group3()[3] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircle> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other[e425]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipole> for VersorOdd {
    fn add_assign(&mut self, other: MysteryDipole) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryDipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group3()[0]),
                (other.group1()[1] + self.group3()[1]),
                (other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryDipoleInversion> for VersorOdd {
    fn add_assign(&mut self, other: MysteryDipoleInversion) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] + self.group3()[0]),
                (other.group1()[1] + self.group3()[1]),
                (other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MysteryQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] + self.group0()[3])]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group3()[0]),
                (other.group0()[2] + self.group3()[1]),
                (other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryVersorOdd> for VersorOdd {
    fn add_assign(&mut self, other: MysteryVersorOdd) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] + self.group0()[3])]),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] + self.group3()[0]),
                (other.group0()[2] + self.group3()[1]),
                (other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleInversionAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e1234])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullSphereAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e1234])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Plane> for VersorOdd {
    fn add_assign(&mut self, other: Plane) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<PlaneOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: PlaneOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNum> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNumAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[2]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNumAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[2]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNumOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for VersorOdd {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Sphere> for VersorOdd {
    fn add_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            (other.group0() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<SphereAtOrigin> for VersorOdd {
    fn add_assign(&mut self, other: SphereAtOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<SphereOnOrigin> for VersorOdd {
    fn add_assign(&mut self, other: SphereOnOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEven> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorOdd {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOdd> for VersorOdd {
    fn add_assign(&mut self, other: VersorOdd) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() + self.group0()),
            // e23, e31, e12, e45
            (other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (other.group3() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e23, e31, e12, e45
            (self.group1() + other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[1]),
                (self.group2()[1] + other.group0()[2]),
                (self.group2()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOddAtInfinity> for VersorOdd {
    fn add_assign(&mut self, other: VersorOddAtInfinity) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e23, e31, e12, e45
            (self.group1() + other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] + other.group0()[1]),
                (self.group2()[1] + other.group0()[2]),
                (self.group2()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() + other.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (self.group2() + other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorOddOrthogonalOrigin> for VersorOdd {
    fn add_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (self.group2() + other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       24        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       40       56        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       36       52        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       28       44        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       34       46        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       25       40        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       34        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<AntiFlatOrigin> for VersorOdd {
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
impl std::ops::BitXor<AntiFlatPoint> for VersorOdd {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       31       40        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       21        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       23        0
    //  no simd       15       29        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       25       41        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       22        0
    //  no simd       12       28        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       16       32        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       26        0
    //  no simd       21       32        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        4       20        0
    fn bitxor(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryQuadNum> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       17       35        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       27        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       22        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNum> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        2       18        0
    fn bitxor(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        5       21        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAtInfinity> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumOnOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiQuadNumOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       17       35        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       33        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       25       41        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for VersorOdd {
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
impl std::ops::BitXor<CircleAtInfinity> for VersorOdd {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       14        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for VersorOdd {
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
impl std::ops::BitXor<CircleOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<CircleOrthogonalOrigin> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       14        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for VersorOdd {
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
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for VersorOdd {
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
impl std::ops::BitXor<CircleRotorAtInfinity> for VersorOdd {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       12        0
    //  no simd        7       15        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<Dipole> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       25       40        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for VersorOdd {
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
impl std::ops::BitXor<DipoleAtInfinity> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       16       25        0
    //  no simd       16       28        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for VersorOdd {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for VersorOdd {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       21       33        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       26        0
    //  no simd       20       32        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for VersorOdd {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       20        0
    //  no simd       15       26        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       24       38        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<DipoleOrthogonalOrigin> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       22       36        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for VersorOdd {
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
impl std::ops::BitXor<FlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       13        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for VersorOdd {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for VersorOdd {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorOdd {
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
impl std::ops::BitXor<LineAtInfinity> for VersorOdd {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for VersorOdd {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorOdd {
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
impl std::ops::BitXor<MotorAtInfinity> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for VersorOdd {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd       90      122        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for VersorOdd {
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
impl std::ops::BitXor<MysteryCircleRotor> for VersorOdd {
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
impl std::ops::BitXor<MysteryDipole> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       16        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       19        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for VersorOdd {
    type Output = MysteryQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       27        0
    //  no simd       22       36        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       19        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       19       35        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for VersorOdd {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for VersorOdd {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for VersorOdd {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for VersorOdd {
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
impl std::ops::BitXor<NullSphereAtOrigin> for VersorOdd {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for VersorOdd {
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
impl std::ops::BitXor<Origin> for VersorOdd {
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
impl std::ops::BitXor<Plane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for VersorOdd {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        6       19        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOrigin> for VersorOdd {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        5       17        0
    fn bitxor(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for VersorOdd {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       11        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOnOrigin> for VersorOdd {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        5       18        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       43        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for VersorOdd {
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
impl std::ops::BitXor<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorOdd {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorOdd {
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
impl std::ops::BitXor<SphereAtOrigin> for VersorOdd {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for VersorOdd {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for VersorOdd {
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
impl std::ops::BitXor<VersorEvenAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       23       35        0
    //  no simd       35       47        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for VersorOdd {
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
impl std::ops::BitXor<VersorEvenOnOrigin> for VersorOdd {
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
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       38       54        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       48       61        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for VersorOdd {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       32       48        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for VersorOdd {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       30        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       23       36        0
    //  no simd       41       54        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for VersorOdd {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiCircleOnOrigin> for VersorOdd {
    fn from(anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotor> for VersorOdd {
    fn from(anti_circle_rotor: AntiCircleRotor) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43], anti_circle_rotor[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12], anti_circle_rotor[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor[e15], anti_circle_rotor[e25], anti_circle_rotor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e41],
                anti_circle_rotor_aligning_origin[e42],
                anti_circle_rotor_aligning_origin[e43],
                anti_circle_rotor_aligning_origin[scalar],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e23],
                anti_circle_rotor_aligning_origin[e31],
                anti_circle_rotor_aligning_origin[e12],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e15],
                anti_circle_rotor_aligning_origin[e25],
                anti_circle_rotor_aligning_origin[e35],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_aligning_origin_at_infinity[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e23],
                anti_circle_rotor_aligning_origin_at_infinity[e31],
                anti_circle_rotor_aligning_origin_at_infinity[e12],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                anti_circle_rotor_aligning_origin_at_infinity[e15],
                anti_circle_rotor_aligning_origin_at_infinity[e25],
                anti_circle_rotor_aligning_origin_at_infinity[e35],
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorAtInfinity> for VersorOdd {
    fn from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_at_infinity[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_circle_rotor_at_infinity[e23],
                anti_circle_rotor_at_infinity[e31],
                anti_circle_rotor_at_infinity[e12],
                anti_circle_rotor_at_infinity[e45],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_circle_rotor_at_infinity[e15], anti_circle_rotor_at_infinity[e25], anti_circle_rotor_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiCircleRotorOnOrigin> for VersorOdd {
    fn from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                anti_circle_rotor_on_origin[e41],
                anti_circle_rotor_on_origin[e42],
                anti_circle_rotor_on_origin[e43],
                anti_circle_rotor_on_origin[scalar],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_circle_rotor_on_origin[e23], anti_circle_rotor_on_origin[e31], anti_circle_rotor_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLine> for VersorOdd {
    fn from(anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_line[e23], anti_line[e31], anti_line[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLineOnOrigin> for VersorOdd {
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMotor> for VersorOdd {
    fn from(anti_motor: AntiMotor) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
        );
    }
}

impl From<AntiMotorOnOrigin> for VersorOdd {
    fn from(anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor_on_origin[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMysteryCircleRotor> for VersorOdd {
    fn from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                anti_mystery_circle_rotor[e23],
                anti_mystery_circle_rotor[e31],
                anti_mystery_circle_rotor[e12],
                anti_mystery_circle_rotor[e45],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiMysteryQuadNum> for VersorOdd {
    fn from(anti_mystery_quad_num: AntiMysteryQuadNum) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_quad_num[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_quad_num[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiQuadNum> for VersorOdd {
    fn from(anti_quad_num: AntiQuadNum) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[e3215]]),
        );
    }
}

impl From<AntiQuadNumAligningOrigin> for VersorOdd {
    fn from(anti_quad_num_aligning_origin: AntiQuadNumAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin[e3215]]),
        );
    }
}

impl From<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    fn from(anti_quad_num_aligning_origin_at_infinity: AntiQuadNumAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin_at_infinity[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin_at_infinity[e3215]]),
        );
    }
}

impl From<AntiQuadNumAtInfinity> for VersorOdd {
    fn from(anti_quad_num_at_infinity: AntiQuadNumAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[e3215]]),
        );
    }
}

impl From<AntiQuadNumOnOrigin> for VersorOdd {
    fn from(anti_quad_num_on_origin: AntiQuadNumOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_on_origin[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    fn from(anti_quad_num_orthogonal_origin: AntiQuadNumOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_orthogonal_origin[e3215]]),
        );
    }
}

impl From<AntiVersorEvenOnOrigin> for VersorOdd {
    fn from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                anti_versor_even_on_origin[e41],
                anti_versor_even_on_origin[e42],
                anti_versor_even_on_origin[e43],
                anti_versor_even_on_origin[scalar],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([anti_versor_even_on_origin[e23], anti_versor_even_on_origin[e31], anti_versor_even_on_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_even_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Dipole> for VersorOdd {
    fn from(dipole: Dipole) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], dipole[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAligningOrigin> for VersorOdd {
    fn from(dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_aligning_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtInfinity> for VersorOdd {
    fn from(dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12], dipole_at_infinity[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtOrigin> for VersorOdd {
    fn from(dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleInversion> for VersorOdd {
    fn from(dipole_inversion: DipoleInversion) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], dipole_inversion[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], dipole_inversion[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion[e4235], dipole_inversion[e4315], dipole_inversion[e4125], dipole_inversion[e3215]]),
        );
    }
}

impl From<DipoleInversionAligningOrigin> for VersorOdd {
    fn from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_aligning_origin[e41], dipole_inversion_aligning_origin[e42], dipole_inversion_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_aligning_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                dipole_inversion_aligning_origin[e15],
                dipole_inversion_aligning_origin[e25],
                dipole_inversion_aligning_origin[e35],
                dipole_inversion_aligning_origin[e1234],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                dipole_inversion_aligning_origin[e4235],
                dipole_inversion_aligning_origin[e4315],
                dipole_inversion_aligning_origin[e4125],
                dipole_inversion_aligning_origin[e3215],
            ]),
        );
    }
}

impl From<DipoleInversionAtInfinity> for VersorOdd {
    fn from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                dipole_inversion_at_infinity[e23],
                dipole_inversion_at_infinity[e31],
                dipole_inversion_at_infinity[e12],
                dipole_inversion_at_infinity[e45],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_inversion_at_infinity[e15], dipole_inversion_at_infinity[e25], dipole_inversion_at_infinity[e35], 0.0]),
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

impl From<DipoleInversionAtOrigin> for VersorOdd {
    fn from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_at_origin[e41], dipole_inversion_at_origin[e42], dipole_inversion_at_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([
                dipole_inversion_at_origin[e15],
                dipole_inversion_at_origin[e25],
                dipole_inversion_at_origin[e35],
                dipole_inversion_at_origin[e1234],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_at_origin[e3215]]),
        );
    }
}

impl From<DipoleInversionOnOrigin> for VersorOdd {
    fn from(dipole_inversion_on_origin: DipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_on_origin[e41], dipole_inversion_on_origin[e42], dipole_inversion_on_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([dipole_inversion_on_origin[e4235], dipole_inversion_on_origin[e4315], dipole_inversion_on_origin[e4125], 0.0]),
        );
    }
}

impl From<DipoleInversionOrthogonalOrigin> for VersorOdd {
    fn from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e41],
                dipole_inversion_orthogonal_origin[e42],
                dipole_inversion_orthogonal_origin[e43],
                0.0,
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e23],
                dipole_inversion_orthogonal_origin[e31],
                dipole_inversion_orthogonal_origin[e12],
                0.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e15],
                dipole_inversion_orthogonal_origin[e25],
                dipole_inversion_orthogonal_origin[e35],
                dipole_inversion_orthogonal_origin[e1234],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, dipole_inversion_orthogonal_origin[e3215]]),
        );
    }
}

impl From<DipoleOnOrigin> for VersorOdd {
    fn from(dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for VersorOdd {
    fn from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatOrigin> for VersorOdd {
    fn from(flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for VersorOdd {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for VersorOdd {
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for VersorOdd {
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
        );
    }
}

impl From<FlectorAtInfinity> for VersorOdd {
    fn from(flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector_at_infinity[e3215]]),
        );
    }
}

impl From<FlectorOnOrigin> for VersorOdd {
    fn from(flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector_on_origin[e4235], flector_on_origin[e4315], flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for VersorOdd {
    fn from(horizon: Horizon) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, horizon[e3215]]),
        );
    }
}

impl From<MysteryDipole> for VersorOdd {
    fn from(mystery_dipole: MysteryDipole) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([mystery_dipole[e23], mystery_dipole[e31], mystery_dipole[e12], mystery_dipole[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryDipoleInversion> for VersorOdd {
    fn from(mystery_dipole_inversion: MysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                mystery_dipole_inversion[e23],
                mystery_dipole_inversion[e31],
                mystery_dipole_inversion[e12],
                mystery_dipole_inversion[e45],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_dipole_inversion[e4235], mystery_dipole_inversion[e4315], mystery_dipole_inversion[e4125], 0.0]),
        );
    }
}

impl From<MysteryVersorOdd> for VersorOdd {
    fn from(mystery_versor_odd: MysteryVersorOdd) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([mystery_versor_odd[e23], mystery_versor_odd[e31], mystery_versor_odd[e12], mystery_versor_odd[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([mystery_versor_odd[e4235], mystery_versor_odd[e4315], mystery_versor_odd[e4125], 0.0]),
        );
    }
}

impl From<NullDipoleAtOrigin> for VersorOdd {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullDipoleInversionAtOrigin> for VersorOdd {
    fn from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_dipole_inversion_at_origin[e41], null_dipole_inversion_at_origin[e42], null_dipole_inversion_at_origin[e43], 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_dipole_inversion_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullSphereAtOrigin> for VersorOdd {
    fn from(null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_sphere_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Plane> for VersorOdd {
    fn from(plane: Plane) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane[e4235], plane[e4315], plane[e4125], plane[e3215]]),
        );
    }
}

impl From<PlaneOnOrigin> for VersorOdd {
    fn from(plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane_on_origin[e4235], plane_on_origin[e4315], plane_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Scalar> for VersorOdd {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, scalar[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Sphere> for VersorOdd {
    fn from(sphere: Sphere) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere[e4235], sphere[e4315], sphere[e4125], sphere[e3215]]),
        );
    }
}

impl From<SphereAtOrigin> for VersorOdd {
    fn from(sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e3215]]),
        );
    }
}

impl From<SphereOnOrigin> for VersorOdd {
    fn from(sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125], 0.0]),
        );
    }
}

impl From<VersorOddAtInfinity> for VersorOdd {
    fn from(versor_odd_at_infinity: VersorOddAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12], versor_odd_at_infinity[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                versor_odd_at_infinity[e4235],
                versor_odd_at_infinity[e4315],
                versor_odd_at_infinity[e4125],
                versor_odd_at_infinity[e3215],
            ]),
        );
    }
}

impl From<VersorOddOrthogonalOrigin> for VersorOdd {
    fn from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                versor_odd_orthogonal_origin[e41],
                versor_odd_orthogonal_origin[e42],
                versor_odd_orthogonal_origin[e43],
                versor_odd_orthogonal_origin[scalar],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e23], versor_odd_orthogonal_origin[e31], versor_odd_orthogonal_origin[e12], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
                versor_odd_orthogonal_origin[e1234],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_orthogonal_origin[e3215]]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       80       96        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       88      104        0
    //  no simd      160      176        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       87      103        0
    //  no simd      144      160        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       96      112        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       61       78        0
    //  no simd      112      129        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       71        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       63       82        0
    //  no simd       96      115        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       41       41        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      224      240        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       69        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       79       96        0
    //  no simd      160      177        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       36        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       46       59        0
    //  no simd      115      128        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       82       98        0
    //  no simd      160      176        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       52       64        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        7        0
    // no simd        0       28        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       30       48        0
    //  no simd       48       72        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       55        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       56       74        0
    //  no simd      113      131        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       64        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       80       96        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorOdd {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       64       80        0
    //  no simd      112      128        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorOdd {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4       12       13        0
    // Totals...
    // yes simd       28       45        0
    //  no simd       64       84        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryCircleRotor> for VersorOdd {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       57       73        0
    //  no simd       96      112        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4       10        0
    // no simd       16       40        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryQuadNum> for VersorOdd {
    fn mul_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       49       71        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       32       48        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       18        0
    // no simd       48       72        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNum> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        6        9        0
    // Totals...
    // yes simd       14       29        0
    //  no simd       32       56        0
    fn mul(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAligningOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       27        0
    //  no simd       16       39        0
    fn mul(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       23        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       35       55        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       17       35        0
    fn mul(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNumOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       17       35        0
    //  no simd       32       56        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       35        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       43        0
    //  no simd       48       67        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       61       79        0
    //  no simd      112      130        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       81       97        0
    //  no simd      144      160        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      128      144        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       57       74        0
    //  no simd       96      113        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       80       96        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       80       96        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       48       64        0
    //  no simd       96      112        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       25       25        0
    // Totals...
    // yes simd       85      101        0
    //  no simd      160      176        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       90      106        0
    //  no simd      144      160        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       66       82        0
    //  no simd       96      112        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       61       78        0
    //  no simd      112      129        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       96      112        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       93      109        0
    //  no simd      144      160        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for VersorOdd {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       63       79        0
    //  no simd       96      112        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       73        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       66       83        0
    //  no simd       96      113        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4       37       37        0
    // Totals...
    // yes simd      113      129        0
    //  no simd      224      240        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       33       33        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      176      192        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       97      113        0
    //  no simd      160      176        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       49       65        0
    //  no simd      112      128        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       53        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       59       72        0
    //  no simd      116      129        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       88      104        0
    //  no simd      160      176        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       31       50        0
    //  no simd       49       68        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       89      105        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        6        0
    // no simd        0       24        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       33       47        0
    //  no simd       48       68        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorOdd {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorOdd {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       36       48        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       60        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       58       78        0
    //  no simd      112      132        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorOdd {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorOdd {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       55       65        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       64        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlectorOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorOdd {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       17        0
    //  no simd        4       23        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorOdd {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        4        0
    // no simd        4       16        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       80       96        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for VersorOdd {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       36       48        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       59       74        0
    //  no simd      113      128        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorOdd {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       28       40        0
    //  no simd       52       64        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       48       64        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      116      148        0
    //    simd2       12       12        0
    //    simd3       52       52        0
    //    simd4       46       46        0
    // Totals...
    // yes simd      226      258        0
    //  no simd      480      512        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       24       40        0
    //  no simd       48       64        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       64       80        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       68        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipole> for VersorOdd {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       69       85        0
    //  no simd       96      112        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for VersorOdd {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       16       32        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       58       74        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       58       74        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for VersorOdd {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorOdd {
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
impl std::ops::Mul<NullDipoleAtOrigin> for VersorOdd {
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
impl std::ops::MulAssign<NullDipoleAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorOdd {
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
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorOdd {
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
impl std::ops::MulAssign<NullSphereAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorOdd {
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
impl std::ops::Mul<Origin> for VersorOdd {
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
impl std::ops::Mul<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       27       44        0
    //  no simd       48       65        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorOdd {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       18        0
    // no simd       48       72        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        7       11        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       32       56        0
    fn mul(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       16       36        0
    fn mul(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       35       48        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       22        0
    //    simd4        3        4        0
    // Totals...
    // yes simd        8       26        0
    //  no simd       17       38        0
    fn mul(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        8       13        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       32       56        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4       12       14        0
    // Totals...
    // yes simd       28       42        0
    //  no simd       64       84        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        4        8        0
    // Totals...
    // yes simd        4       16        0
    //  no simd       16       40        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorOdd {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       11       13        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       64       84        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for VersorOdd {
    fn mul_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       16       40        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       52       68        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       47       47        0
    // Totals...
    // yes simd       99      115        0
    //  no simd      240      256        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       33       33        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       73        0
    //    simd4       30       30        0
    // Totals...
    // yes simd       86      103        0
    //  no simd      176      193        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       55       71        0
    //  no simd      112      128        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       29        0
    //    simd4       24       25        0
    // Totals...
    // yes simd       43       54        0
    //  no simd      115      129        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       33       33        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       76       92        0
    //    simd4       41       41        0
    // Totals...
    // yes simd      117      133        0
    //  no simd      240      256        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorOdd {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       98      114        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for VersorOdd {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       31       31        0
    // Totals...
    // yes simd       83       99        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for VersorOdd {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorOdd {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn neg(self) -> Self {
        let negation = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(-1.0)),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn not(self) -> Self::Output {
        let right_dual = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group2()[0] * -1.0), (self.group2()[1] * -1.0), (self.group2()[2] * -1.0), self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group3()[0] * -1.0), (self.group3()[1] * -1.0), (self.group3()[2] * -1.0), self.group2()[3]]),
        );
        return right_dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group2()[3] * -1.0)]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLine> for VersorOdd {
    fn sub_assign(&mut self, other: AntiLine) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group1()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMotor) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group1()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e31])]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryCircleRotor> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e31])]),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMysteryQuadNum> for VersorOdd {
    fn sub_assign(&mut self, other: AntiMysteryQuadNum) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNum> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNum) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAligningOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNumAligningOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAligningOriginAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNumAligningOriginAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNumAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNumOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNumOrthogonalOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiVersorEvenOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Circle> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Dipole> for VersorOdd {
    fn sub_assign(&mut self, other: Dipole) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAligningOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleAligningOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleAtOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversion> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversion) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            (-other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAligningOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversionAligningOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            (-other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversionAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group2() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (-other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversionAtOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            (-other.group1() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        0        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[1] + self.group3()[0]),
                (-other.group1()[2] + self.group3()[1]),
                (-other.group1()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversionOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[1] + self.group3()[0]),
                (-other.group1()[2] + self.group3()[1]),
                (-other.group1()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       11        0        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleInversionOrthogonalOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
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
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DipoleOrthogonalOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: DipoleOrthogonalOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
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
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e45])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e45])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPoint> for VersorOdd {
    fn sub_assign(&mut self, other: FlatPoint) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group1() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Flector> for VersorOdd {
    fn sub_assign(&mut self, other: Flector) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (-other.group1() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: FlectorAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group3()[0]),
                (-other.group0()[2] + self.group3()[1]),
                (-other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<FlectorOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: FlectorOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group3()[0]),
                (-other.group0()[2] + self.group3()[1]),
                (-other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Horizon> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e3215])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Horizon> for VersorOdd {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e3215])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Infinity> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other[e5] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        2        0
    //    simd3        2        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6        6        0
    //  no simd       16       16        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(-other.group0()[0] + self.group0()[3]), (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) - other.group3()),
            // e15, e25, e35
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) - other.group4()),
            // e23, e31, e12
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]) - other.group9()),
            // e3215
            (self.group3()[3] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other[e425] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipole> for VersorOdd {
    fn sub_assign(&mut self, other: MysteryDipole) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group3()[0]),
                (-other.group1()[1] + self.group3()[1]),
                (-other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryDipoleInversion> for VersorOdd {
    fn sub_assign(&mut self, other: MysteryDipoleInversion) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            (-other.group0() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group1()[0] + self.group3()[0]),
                (-other.group1()[1] + self.group3()[1]),
                (-other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: MysteryQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[0] + self.group0()[3])]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group3()[0]),
                (-other.group0()[2] + self.group3()[1]),
                (-other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryVersorOdd> for VersorOdd {
    fn sub_assign(&mut self, other: MysteryVersorOdd) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[0] + self.group0()[3])]),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[1] + self.group3()[0]),
                (-other.group0()[2] + self.group3()[1]),
                (-other.group0()[3] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleInversionAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e1234])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullSphereAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e1234])]),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Plane> for VersorOdd {
    fn sub_assign(&mut self, other: Plane) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<PlaneOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: PlaneOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNum> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: QuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: QuadNumAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[2] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOriginAtInfinity> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: QuadNumAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[2] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOnOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: QuadNumOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for VersorOdd {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            self.group2(),
            // e4235, e4315, e4125, e3215
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Sphere> for VersorOdd {
    fn sub_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            (-other.group0() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<SphereAtOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: SphereAtOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<SphereOnOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: SphereOnOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEven> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorOdd {
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
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([self.group2()[3], self.group3()[0], self.group3()[1], self.group3()[2]]),
            // e3215
            self.group3()[3],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOdd> for VersorOdd {
    fn sub_assign(&mut self, other: VersorOdd) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-other.group0() + self.group0()),
            // e23, e31, e12, e45
            (-other.group1() + self.group1()),
            // e15, e25, e35, e1234
            (-other.group2() + self.group2()),
            // e4235, e4315, e4125, e3215
            (-other.group3() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e23, e31, e12, e45
            (self.group1() - other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[1]),
                (self.group2()[1] - other.group0()[2]),
                (self.group2()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOddAtInfinity> for VersorOdd {
    fn sub_assign(&mut self, other: VersorOddAtInfinity) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e23, e31, e12, e45
            (self.group1() - other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] - other.group0()[1]),
                (self.group2()[1] - other.group0()[2]),
                (self.group2()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() - other.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (self.group2() - other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorOddOrthogonalOrigin> for VersorOdd {
    fn sub_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            (self.group2() - other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        *self = subtraction;
    }
}

impl TryFrom<MultiVector> for VersorOdd {
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOdd { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}
