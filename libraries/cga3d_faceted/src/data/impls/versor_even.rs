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
// Total Implementations: 509
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        12      16       0
//  Maximum:       233     265       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       4       0
//  Average:        23      28       0
//  Maximum:       480     512       0
impl std::ops::Add<AntiCircleOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotorAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotorAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]]) + self.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]]) + self.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group2()[0] + self.group3()[0]),
                (other.group2()[1] + self.group3()[1]),
                (other.group2()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group2()[0] + self.group3()[0]),
                (other.group2()[1] + self.group3()[1]),
                (other.group2()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group2()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group2()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for VersorEven {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + self.group2()),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<AntiFlector> for VersorEven {
    fn add_assign(&mut self, other: AntiFlector) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + self.group2()),
            // e1, e2, e3, e4
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
impl std::ops::Add<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<AntiFlectorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Add<AntiLine> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiLineOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiMotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiMotorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiMysteryCircleRotor> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e31], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for VersorEven {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Add<AntiMysteryQuadNum> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<AntiPlane> for VersorEven {
    fn add_assign(&mut self, other: AntiPlane) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::Add<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<AntiPlaneOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Add<AntiQuadNum> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[2], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e12345])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for VersorEven {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e12345])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group0() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (other.group0() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<AntiVersorRoundPointOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Circle> for VersorEven {
    fn add_assign(&mut self, other: Circle) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotor> for VersorEven {
    fn add_assign(&mut self, other: CircleRotor) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Dipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversionAligningOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversionAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversionAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversionOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<DipoleOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<FlatOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<FlatPoint> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<FlatPointAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<Flector> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<FlectorAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<FlectorOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<Horizon> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<Infinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Infinity> for VersorEven {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for VersorEven {
    fn add_assign(&mut self, other: Line) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: LineAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: LineOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (other.group1() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Motor> for VersorEven {
    fn add_assign(&mut self, other: Motor) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (other.group1() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group0() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (other.group0() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MultiVector> for VersorEven {
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
            Simd32x2::from([other.group0()[0], (other.group0()[1] + self.group0()[3])]),
            // e1, e2, e3, e4
            (other.group1() + self.group3()),
            // e5
            (self.group2()[3] + other[e1]),
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            (other.group6() + self.group1()),
            // e423, e431, e412
            (Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) + other.group7()),
            // e235, e315, e125
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) + other.group8()),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryCircle> for VersorEven {
    fn add_assign(&mut self, other: MysteryCircle) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e425])]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for VersorEven {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e425])]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryDipole> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<MysteryDipoleInversion> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: MysteryQuadNum) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MysteryQuadNum> for VersorEven {
    fn add_assign(&mut self, other: MysteryQuadNum) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] + self.group0()[3])]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<MysteryVersorEven> for VersorEven {
    fn add_assign(&mut self, other: MysteryVersorEven) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] + self.group0()[3])]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Add<MysteryVersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<MysteryVersorRoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorRoundPoint) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<MysteryVersorRoundPoint> for VersorEven {
    fn add_assign(&mut self, other: MysteryVersorRoundPoint) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Add<MysteryVersorSphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorSphere) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<NullCircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<NullDipoleInversionAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<NullSphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<NullVersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[3] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Origin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e4])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Origin> for VersorEven {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e4])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Plane> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<PlaneOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<QuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: QuadNum) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNum> for VersorEven {
    fn add_assign(&mut self, other: QuadNum) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: QuadNumAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: QuadNumAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<QuadNumOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e2])]),
            // e1, e2, e3, e4
            (other.group0() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<RoundPoint> for VersorEven {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e2])]),
            // e1, e2, e3, e4
            (other.group0() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Scalar> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<Sphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<SphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<SphereOnOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            (other.group2() + self.group2()),
            // e1, e2, e3, e4
            (other.group3() + self.group3()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEven> for VersorEven {
    fn add_assign(&mut self, other: VersorEven) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            (other.group2() + self.group2()),
            // e1, e2, e3, e4
            (other.group3() + self.group3()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() + other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAligningOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() + other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (self.group2() + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group3()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() + other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() + other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            (self.group3() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() + other.group1()),
            // e1, e2, e3, e4
            (self.group3() + other.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorOdd> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<VersorOddAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<VersorOddOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<VersorRoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        6        0        0
    fn add(self, other: VersorRoundPoint) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            (self.group3() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorRoundPoint> for VersorEven {
    fn add_assign(&mut self, other: VersorRoundPoint) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            (self.group3() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorRoundPointAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorRoundPointAligningOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorRoundPointAligningOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: VersorRoundPointAligningOriginAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorRoundPointAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn add(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::AddAssign<VersorRoundPointAtInfinity> for VersorEven {
    fn add_assign(&mut self, other: VersorRoundPointAtInfinity) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::Add<VersorRoundPointOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<VersorRoundPointOnOrigin> for VersorEven {
    fn add_assign(&mut self, other: VersorRoundPointOnOrigin) {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group3()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<VersorSphere> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorSphere) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[1], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[0], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorSphereAtInfinity> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e4315], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Add<VersorSphereOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    fn add(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[2], self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::BitXor<AntiCircleOnOrigin> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd3        2        3        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       13       24        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       30       43        0
    //  no simd       36       52        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       28       44        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        0
    //    simd3        1        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       48       60        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        0        1        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       34       46        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       30        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       17       34        0
    //  no simd       25       44        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       26        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       22       38        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for VersorEven {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       28       40        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       21        0
    //  no simd       15       29        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       12        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       41        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       19       32        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       18       35        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::BitXorAssign<AntiMysteryQuadNum> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       20        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       17       35        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       10       27        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        6       22        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNum> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        5       21        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNumAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for VersorEven {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       26        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       29        0
    //  no simd       17       36        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       25       41        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorRoundPointOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorRoundPointOnOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: AntiVersorRoundPointOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        9       14        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for VersorEven {
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
impl std::ops::BitXor<CircleOrthogonalOrigin> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        9       14        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       13       18        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       10        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        9       14        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for VersorEven {
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
impl std::ops::BitXor<Dipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        2        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       29       40        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for VersorEven {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       19        0
    //    simd3        2        3        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       18       28        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       23        0
    //  no simd       17       28        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for VersorEven {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       14       24        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd3        2        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       27        0
    //  no simd       37       45        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for VersorEven {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd3        2        3        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       16       21        0
    //  no simd       26       33        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       21       32        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for VersorEven {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       16        0
    //  no simd       19       26        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for VersorEven {
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
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       23        0
    //    simd3        1        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       30       38        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for VersorEven {
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
impl std::ops::BitXor<DipoleOrthogonalOrigin> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        2        3        0
    // Totals...
    // yes simd       21       30        0
    //  no simd       25       36        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for VersorEven {
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
impl std::ops::BitXor<FlatPoint> for VersorEven {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        9       16        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for VersorEven {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       12        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for VersorEven {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        1        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       10        0
    //  no simd       16       20        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for VersorEven {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        2        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        9       13        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for VersorEven {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       10        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for VersorEven {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for VersorEven {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       78        0
    //  no simd       90      121        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        5       19        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       20        0
    //  no simd        8       22        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for VersorEven {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       24        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       18       35        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       20        0
    //  no simd       25       35        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for VersorEven {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorRoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       23        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       25        0
    //  no simd       10       30        0
    fn bitxor(self, other: MysteryVersorRoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorSphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       19        0
    fn bitxor(self, other: MysteryVersorSphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorSphere> for VersorEven {
    fn bitxor_assign(&mut self, other: MysteryVersorSphere) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for VersorEven {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for VersorEven {
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
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for VersorEven {
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
impl std::ops::BitXor<NullSphereAtOrigin> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for VersorEven {
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
impl std::ops::BitXor<Origin> for VersorEven {
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
impl std::ops::BitXor<Plane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       15        0
    //  no simd        6       21        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       10        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3       15        0
    //  no simd        6       21        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       25       40        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       20        0
    //  no simd        4       23        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for VersorEven {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       22        0
    //    simd3        1        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       48       60        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       16       31        0
    //  no simd       22       37        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        0        1        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       34       46        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       16       31        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for VersorEven {
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
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd3        1        2        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       22       29        0
    //  no simd       42       54        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       28        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       23       33        0
    //  no simd       35       48        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       34        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       38       54        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       25       40        0
    fn bitxor(self, other: VersorRoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAligningOrigin> for VersorEven {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       19        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       20        0
    //  no simd        4       23        0
    fn bitxor(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointAtInfinity> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       25        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       17       32        0
    fn bitxor(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorRoundPointOnOrigin> for VersorEven {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       12        0
    //  no simd        5       21        0
    fn bitxor(self, other: VersorSphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorSphere> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorSphere) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphereAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4       11        0
    //  no simd        4       20        0
    fn bitxor(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorSphereAtInfinity> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorSphereOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        2       18        0
    fn bitxor(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorSphereOrthogonalOrigin> for VersorEven {
    fn bitxor_assign(&mut self, other: VersorSphereOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiDipoleInversion> for VersorEven {
    fn from(anti_dipole_inversion: AntiDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_inversion[e423], anti_dipole_inversion[e431], anti_dipole_inversion[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435], anti_dipole_inversion[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], anti_dipole_inversion[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3], anti_dipole_inversion[e4]]),
        );
    }
}

impl From<AntiDipoleInversionAtInfinity> for VersorEven {
    fn from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
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
            // e1, e2, e3, e4
            Simd32x4::from([anti_dipole_inversion_at_infinity[e1], anti_dipole_inversion_at_infinity[e2], anti_dipole_inversion_at_infinity[e3], 0.0]),
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for VersorEven {
    fn from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                anti_dipole_inversion_on_origin[e1],
                anti_dipole_inversion_on_origin[e2],
                anti_dipole_inversion_on_origin[e3],
                anti_dipole_inversion_on_origin[e4],
            ]),
        );
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e423],
                anti_dipole_inversion_orthogonal_origin[e431],
                anti_dipole_inversion_orthogonal_origin[e412],
                0.0,
            ]),
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
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_orthogonal_origin[e4]]),
        );
    }
}

impl From<AntiDipoleOnOrigin> for VersorEven {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEven {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], 0.0]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEven {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiMysteryDipoleInversion> for VersorEven {
    fn from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_mystery_dipole_inversion[e415],
                anti_mystery_dipole_inversion[e425],
                anti_mystery_dipole_inversion[e435],
                anti_mystery_dipole_inversion[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_mystery_dipole_inversion[e1], anti_mystery_dipole_inversion[e2], anti_mystery_dipole_inversion[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], 0.0]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEven {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, anti_scalar[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiSphereOnOrigin> for VersorEven {
    fn from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], anti_sphere_on_origin[e4]]),
        );
    }
}

impl From<Circle> for VersorEven {
    fn from(circle: Circle) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle[e423], circle[e431], circle[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAligningOrigin> for VersorEven {
    fn from(circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for VersorEven {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtOrigin> for VersorEven {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOnOrigin> for VersorEven {
    fn from(circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for VersorEven {
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotor> for VersorEven {
    fn from(circle_rotor: CircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412], circle_rotor[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAligningOrigin> for VersorEven {
    fn from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                circle_rotor_aligning_origin[e423],
                circle_rotor_aligning_origin[e431],
                circle_rotor_aligning_origin[e412],
                circle_rotor_aligning_origin[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor_aligning_origin[e235], circle_rotor_aligning_origin[e315], circle_rotor_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, circle_rotor_aligning_origin_at_infinity[e12345]]),
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
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorAtInfinity> for VersorEven {
    fn from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, circle_rotor_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor_at_infinity[e235], circle_rotor_at_infinity[e315], circle_rotor_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleRotorOnOrigin> for VersorEven {
    fn from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_rotor_on_origin[e423], circle_rotor_on_origin[e431], circle_rotor_on_origin[e412], circle_rotor_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for VersorEven {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Line> for VersorEven {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for VersorEven {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for VersorEven {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEven {
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for VersorEven {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorOnOrigin> for VersorEven {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryCircle> for VersorEven {
    fn from(mystery_circle: MysteryCircle) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435], mystery_circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryCircleRotor> for VersorEven {
    fn from(mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_circle_rotor[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435], mystery_circle_rotor[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryQuadNum> for VersorEven {
    fn from(mystery_quad_num: MysteryQuadNum) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_quad_num[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, mystery_quad_num[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryVersorEven> for VersorEven {
    fn from(mystery_versor_even: MysteryVersorEven) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_even[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435], mystery_versor_even[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
        );
    }
}

impl From<MysteryVersorRoundPoint> for VersorEven {
    fn from(mystery_versor_round_point: MysteryVersorRoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_round_point[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([mystery_versor_round_point[e1], mystery_versor_round_point[e2], mystery_versor_round_point[e3], 0.0]),
        );
    }
}

impl From<NullCircleAtOrigin> for VersorEven {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEven {
    fn from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_even_at_origin[e4]]),
        );
    }
}

impl From<Origin> for VersorEven {
    fn from(origin: Origin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, origin[e4]]),
        );
    }
}

impl From<QuadNum> for VersorEven {
    fn from(quad_num: QuadNum) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num[e4]]),
        );
    }
}

impl From<QuadNumAtInfinity> for VersorEven {
    fn from(quad_num_at_infinity: QuadNumAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<QuadNumOrthogonalOrigin> for VersorEven {
    fn from(quad_num_orthogonal_origin: QuadNumOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_orthogonal_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, quad_num_orthogonal_origin[e4]]),
        );
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e4]]),
        );
    }
}

impl From<RoundPointAtOrigin> for VersorEven {
    fn from(round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e4]]),
        );
    }
}

impl From<VersorEvenAligningOrigin> for VersorEven {
    fn from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                versor_even_aligning_origin[e423],
                versor_even_aligning_origin[e431],
                versor_even_aligning_origin[e412],
                versor_even_aligning_origin[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e4]]),
        );
    }
}

impl From<VersorEvenAtInfinity> for VersorEven {
    fn from(versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125], versor_even_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
        );
    }
}

impl From<VersorEvenAtOrigin> for VersorEven {
    fn from(versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e4]]),
        );
    }
}

impl From<VersorEvenOnOrigin> for VersorEven {
    fn from(versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412], versor_even_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e4]]),
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for VersorEven {
    fn from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e4],
            ]),
        );
    }
}

impl From<VersorRoundPoint> for VersorEven {
    fn from(versor_round_point: VersorRoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_round_point[e1], versor_round_point[e2], versor_round_point[e3], versor_round_point[e4]]),
        );
    }
}

impl From<VersorRoundPointAligningOrigin> for VersorEven {
    fn from(versor_round_point_aligning_origin: VersorRoundPointAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_aligning_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_aligning_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_aligning_origin[e4]]),
        );
    }
}

impl From<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    fn from(versor_round_point_aligning_origin_at_infinity: VersorRoundPointAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_aligning_origin_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_aligning_origin_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<VersorRoundPointAtInfinity> for VersorEven {
    fn from(versor_round_point_at_infinity: VersorRoundPointAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_round_point_at_infinity[e1], versor_round_point_at_infinity[e2], versor_round_point_at_infinity[e3], 0.0]),
        );
    }
}

impl From<VersorRoundPointOnOrigin> for VersorEven {
    fn from(versor_round_point_on_origin: VersorRoundPointOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_round_point_on_origin[e4]]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       80       96        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       91      107        0
    //  no simd      160      176        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       90      106        0
    //  no simd      144      160        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       54       70        0
    //  no simd       96      112        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       53       68        0
    //  no simd      113      128        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       57       73        0
    //  no simd       96      112        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiCircleRotorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       46       47        0
    // Totals...
    // yes simd       86      103        0
    //  no simd      224      244        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       67        0
    //    simd4       28       28        0
    // Totals...
    // yes simd       76       95        0
    //  no simd      160      179        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       59       74        0
    //  no simd      113      128        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       88      104        0
    //  no simd      160      176        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       48        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       34       53        0
    //  no simd       49       68        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       32        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       51       64        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       32        0
    //    simd4       23       24        0
    // Totals...
    // yes simd       50       56        0
    //  no simd      119      128        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       16        0
    // no simd       48       64        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       80       96        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLine> for VersorEven {
    fn mul_assign(&mut self, other: AntiLine) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiLineOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiLineOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       53        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       55       72        0
    //  no simd      112      129        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::MulAssign<AntiMotorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::MulAssign<AntiMysteryCircleRotor> for VersorEven {
    fn mul_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       54       70        0
    //  no simd       96      112        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        4        8        0
    // Totals...
    // yes simd        4       16        0
    //  no simd       16       40        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiMysteryQuadNum> for VersorEven {
    fn mul_assign(&mut self, other: AntiMysteryQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       16       22        0
    //  no simd       55       64        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       32       48        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4       11       16        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       48       72        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNum> for VersorEven {
    fn mul_assign(&mut self, other: AntiQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       32       48        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiQuadNumAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        7       12        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       32       56        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNumOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiQuadNumOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       18       32        0
    //  no simd       48       68        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       52       68        0
    //  no simd      112      128        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorEvenOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       16       32        0
    fn mul(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: AntiVersorRoundPointAligningOriginAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorRoundPointOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd4        4        7        0
    // Totals...
    // yes simd        5       18        0
    //  no simd       17       39        0
    fn mul(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiVersorRoundPointOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: AntiVersorRoundPointOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       84      100        0
    //  no simd      144      160        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       80       96        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      128      144        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       59        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       54       73        0
    //  no simd       96      115        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       80       96        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       71       87        0
    //  no simd       80       96        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       60       76        0
    //  no simd       96      112        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       56        0
    //    simd4       29       31        0
    // Totals...
    // yes simd       73       87        0
    //  no simd      160      180        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       87      103        0
    //  no simd      144      160        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       60       76        0
    //  no simd       96      112        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       39        0
    //    simd4       22       23        0
    // Totals...
    // yes simd       49       62        0
    //  no simd      115      131        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       64       80        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       72       88        0
    //  no simd       96      112        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       87      103        0
    //  no simd      144      160        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Dipole> for VersorEven {
    fn mul_assign(&mut self, other: Dipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4       16       17        0
    // Totals...
    // yes simd       48       61        0
    //  no simd       96      112        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       49       64        0
    //  no simd       97      112        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       60        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       56       69        0
    //  no simd       80       96        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       68        0
    //    simd4       42       43        0
    // Totals...
    // yes simd       98      111        0
    //  no simd      224      240        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       35       35        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      176      192        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAligningOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAligningOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4       33       34        0
    // Totals...
    // yes simd       61       75        0
    //  no simd      160      177        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       21       22        0
    // Totals...
    // yes simd       49       62        0
    //  no simd      112      128        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       40        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       49       62        0
    //  no simd      115      128        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       82       98        0
    //  no simd      160      176        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleInversionOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleInversionOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd4        9       13        0
    // Totals...
    // yes simd       24       33        0
    //  no simd       51       72        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       88        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      128      144        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DipoleOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: DipoleOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        8        0
    // no simd        0       32        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatOrigin> for VersorEven {
    fn mul_assign(&mut self, other: FlatOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       48       64        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<FlatPoint> for VersorEven {
    fn mul_assign(&mut self, other: FlatPoint) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       36       48        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       47       62        0
    //  no simd      113      128        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for VersorEven {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       52       64        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::MulAssign<FlectorOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: FlectorOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for VersorEven {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        4        0
    // no simd        4       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        4       21        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for VersorEven {
    type Output = VersorOdd;
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
impl std::ops::Mul<LineAtInfinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       37       48        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       48        0
    //    simd4       19       20        0
    // Totals...
    // yes simd       58       68        0
    //  no simd      115      128        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for VersorEven {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       23       28        0
    //  no simd       56       64        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for VersorEven {
    type Output = VersorOdd;
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
impl std::ops::Mul<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      164        0
    //    simd2        4        4        0
    //    simd3       48       48        0
    //    simd4       49       49        0
    // Totals...
    // yes simd      233      265        0
    //  no simd      480      512        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4       10       14        0
    // Totals...
    // yes simd       18       26        0
    //  no simd       48       68        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       14       19        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       64       92        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for VersorEven {
    type Output = VersorEven;
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
impl std::ops::MulAssign<MysteryDipole> for VersorEven {
    fn mul_assign(&mut self, other: MysteryDipole) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       17       18        0
    // Totals...
    // yes simd       45       58        0
    //  no simd       96      112        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryDipoleInversion> for VersorEven {
    fn mul_assign(&mut self, other: MysteryDipoleInversion) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       16       40        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4       24       26        0
    // Totals...
    // yes simd       40       50        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4       24       25        0
    // Totals...
    // yes simd       40       53        0
    //  no simd      112      128        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorRoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       18        0
    // no simd       48       72        0
    fn mul(self, other: MysteryVersorRoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorSphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       14        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       48       72        0
    fn mul(self, other: MysteryVersorSphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MysteryVersorSphere> for VersorEven {
    fn mul_assign(&mut self, other: MysteryVersorSphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for VersorEven {
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
impl std::ops::Mul<NullDipoleAtOrigin> for VersorEven {
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
impl std::ops::MulAssign<NullDipoleAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullDipoleAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for VersorEven {
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
impl std::ops::MulAssign<NullDipoleInversionAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullDipoleInversionAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for VersorEven {
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
impl std::ops::MulAssign<NullSphereAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: NullSphereAtOrigin) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for VersorEven {
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
impl std::ops::Mul<Origin> for VersorEven {
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
impl std::ops::Mul<Plane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       21       41        0
    //  no simd       48       68        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for VersorEven {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       32       48        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<PlaneOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: PlaneOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       21       32        0
    //  no simd       48       68        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       33       48        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       32       56        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4       14       16        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       64       84        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for VersorEven {
    type Output = VersorOdd;
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
impl std::ops::Mul<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for VersorEven {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       13       15        0
    // Totals...
    // yes simd       25       39        0
    //  no simd       64       84        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Sphere> for VersorEven {
    fn mul_assign(&mut self, other: Sphere) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4       10        0
    // no simd       16       40        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereAtOrigin> for VersorEven {
    fn mul_assign(&mut self, other: SphereAtOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4       12       14        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       52       71        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<SphereOnOrigin> for VersorEven {
    fn mul_assign(&mut self, other: SphereOnOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for VersorEven {
    type Output = VersorOdd;
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
impl std::ops::Mul<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       32       32        0
    // Totals...
    // yes simd       80       96        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4       34       35        0
    // Totals...
    // yes simd       74       87        0
    //  no simd      176      192        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       52       68        0
    //  no simd      112      128        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       56        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       62       74        0
    //  no simd      116      128        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorOdd;
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
impl std::ops::Mul<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       47       47        0
    // Totals...
    // yes simd       99      115        0
    //  no simd      240      256        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOdd> for VersorEven {
    fn mul_assign(&mut self, other: VersorOdd) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       53        0
    //    simd4       35       35        0
    // Totals...
    // yes simd       71       88        0
    //  no simd      176      193        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       30       30        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      176      192        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorOddOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4       18       21        0
    // Totals...
    // yes simd       26       41        0
    //  no simd       80      104        0
    fn mul(self, other: VersorRoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAligningOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       32       56        0
    fn mul(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       33        0
    //  no simd       16       36        0
    fn mul(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointAtInfinity> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       43       59        0
    //  no simd       64       80        0
    fn mul(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorRoundPointOnOrigin> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       23        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        8       28        0
    //  no simd       17       43        0
    fn mul(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphere> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       17       19        0
    // Totals...
    // yes simd       29       43        0
    //  no simd       80      100        0
    fn mul(self, other: VersorSphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorSphere> for VersorEven {
    fn mul_assign(&mut self, other: VersorSphere) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphereAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       33        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       28       45        0
    //  no simd       64       81        0
    fn mul(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorSphereAtInfinity> for VersorEven {
    fn mul_assign(&mut self, other: VersorSphereAtInfinity) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorSphereOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        8       14        0
    // no simd       32       56        0
    fn mul(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<VersorSphereOrthogonalOrigin> for VersorEven {
    fn mul_assign(&mut self, other: VersorSphereOrthogonalOrigin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for VersorEven {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn neg(self) -> Self {
        let negation = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            (self.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiCircleRotor> for VersorEven {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for VersorEven {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for VersorEven {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiCircleRotorAtInfinity> for VersorEven {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiCircleRotorOnOrigin> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]]) + self.group2()),
            // e1, e2, e3, e4
            (-Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]]) + self.group2()),
            // e1, e2, e3, e4
            (-Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group3()[0]),
                (-other.group2()[1] + self.group3()[1]),
                (-other.group2()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group3()[0]),
                (-other.group2()[1] + self.group3()[1]),
                (-other.group2()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (-swizzle!(other.group1(), 1, 2, 3, 0) + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (-swizzle!(other.group1(), 1, 2, 3, 0) + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group2()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group2()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + self.group2()),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<AntiFlector> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlector) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) + self.group2()),
            // e1, e2, e3, e4
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
impl std::ops::Sub<AntiFlectorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<AntiFlectorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Sub<AntiLine> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiLineOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiMotor> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiMotorOnOrigin> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiMysteryCircleRotor> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        5        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e31] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiMysteryDipoleInversion> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for VersorEven {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Sub<AntiMysteryQuadNum> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiPlane> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<AntiPlane> for VersorEven {
    fn sub_assign(&mut self, other: AntiPlane) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[3] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::Sub<AntiPlaneOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<AntiPlaneOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Sub<AntiQuadNum> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e12345])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for VersorEven {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e12345])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (-other.group0() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            (-other.group0() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiVersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiVersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<AntiVersorRoundPointOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiVersorRoundPointOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Circle> for VersorEven {
    fn sub_assign(&mut self, other: Circle) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotor) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Dipole> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleAligningOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleAtOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversion> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversionAligningOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversionAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversionAtOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversionOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<DipoleOrthogonalOrigin> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<FlatOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<FlatPoint> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<FlatPointAtInfinity> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<Flector> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<FlectorAtInfinity> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<FlectorOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<Horizon> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<Infinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Infinity> for VersorEven {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for VersorEven {
    fn sub_assign(&mut self, other: Line) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (-other.group1() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Motor> for VersorEven {
    fn sub_assign(&mut self, other: Motor) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (-other.group1() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (-other.group0() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (-other.group0() + self.group2()),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MultiVector> for VersorEven {
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
            Simd32x2::from([(other.group0()[0] * -1.0), (-other.group0()[1] + self.group0()[3])]),
            // e1, e2, e3, e4
            (-other.group1() + self.group3()),
            // e5
            (self.group2()[3] - other[e1]),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (-other.group6() + self.group1()),
            // e423, e431, e412
            (Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) - other.group7()),
            // e235, e315, e125
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) - other.group8()),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryCircle> for VersorEven {
    fn sub_assign(&mut self, other: MysteryCircle) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e425])]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for VersorEven {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e425])]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<MysteryDipoleInversion> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: MysteryQuadNum) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MysteryQuadNum> for VersorEven {
    fn sub_assign(&mut self, other: MysteryQuadNum) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[0] + self.group0()[3])]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<MysteryVersorEven> for VersorEven {
    fn sub_assign(&mut self, other: MysteryVersorEven) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[0] + self.group0()[3])]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Sub<MysteryVersorOdd> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<MysteryVersorRoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MysteryVersorRoundPoint) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<MysteryVersorRoundPoint> for VersorEven {
    fn sub_assign(&mut self, other: MysteryVersorRoundPoint) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
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
impl std::ops::Sub<MysteryVersorSphere> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MysteryVersorSphere) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<NullCircleAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<NullDipoleInversionAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<NullSphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<NullVersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[3] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Origin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e4])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Origin> for VersorEven {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e4])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Plane> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<PlaneOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<QuadNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: QuadNum) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNum> for VersorEven {
    fn sub_assign(&mut self, other: QuadNum) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[3] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: QuadNumAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: QuadNumAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[1] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<QuadNumOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: QuadNumOrthogonalOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[2] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e2])]),
            // e1, e2, e3, e4
            (-other.group0() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<RoundPoint> for VersorEven {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e2])]),
            // e1, e2, e3, e4
            (-other.group0() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Scalar> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<Sphere> for VersorEven {
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
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<SphereAtOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<SphereOnOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        4        0        0
    // no simd       16        0        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            (-other.group2() + self.group2()),
            // e1, e2, e3, e4
            (-other.group3() + self.group3()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEven> for VersorEven {
    fn sub_assign(&mut self, other: VersorEven) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            (-other.group2() + self.group2()),
            // e1, e2, e3, e4
            (-other.group3() + self.group3()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAligningOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group3()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (self.group2() - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group3()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() - other.group0()),
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
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       12        0        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            (self.group3() - other.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            (self.group2() - other.group1()),
            // e1, e2, e3, e4
            (self.group3() - other.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<VersorOddAtInfinity> for VersorEven {
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
            Simd32x2::from([(other.group0()[0] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<VersorOddOrthogonalOrigin> for VersorEven {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<VersorRoundPoint> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        6        0        0
    fn sub(self, other: VersorRoundPoint) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            (self.group3() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorRoundPoint> for VersorEven {
    fn sub_assign(&mut self, other: VersorRoundPoint) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            (self.group3() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAligningOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: VersorRoundPointAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorRoundPointAligningOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorRoundPointAligningOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[2] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: VersorRoundPointAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorRoundPointAligningOriginAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: VersorRoundPointAligningOriginAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
            self.group3(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorRoundPointAtInfinity> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        0        0
    fn sub(self, other: VersorRoundPointAtInfinity) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::SubAssign<VersorRoundPointAtInfinity> for VersorEven {
    fn sub_assign(&mut self, other: VersorRoundPointAtInfinity) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group1()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group1()[0] + self.group2()[3])]),
            // e1, e2, e3, e4
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
impl std::ops::Sub<VersorRoundPointOnOrigin> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: VersorRoundPointOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<VersorRoundPointOnOrigin> for VersorEven {
    fn sub_assign(&mut self, other: VersorRoundPointOnOrigin) {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group0()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group3()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<VersorSphere> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn sub(self, other: VersorSphere) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[1] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[0], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorSphereAtInfinity> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn sub(self, other: VersorSphereAtInfinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e4315] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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
impl std::ops::Sub<VersorSphereOrthogonalOrigin> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: VersorSphereOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e4
            self.group3(),
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
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
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

impl TryFrom<MultiVector> for VersorEven {
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
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e4]]),
        ));
    }
}
