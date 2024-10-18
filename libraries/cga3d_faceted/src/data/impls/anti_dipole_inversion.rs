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
// Total Implementations: 427
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        13      17       0
//  Maximum:       235     267       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       5       0
//  Average:        23      29       0
//  Maximum:       448     480       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e4
            (other.group2() + self.group2()),
            // e1, e2, e3, e5
            (other.group3() + self.group3()),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversion> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e4
            (other.group2() + self.group2()),
            // e1, e2, e3, e5
            (other.group3() + self.group3()),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() + other.group2()),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() + other.group2()),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group1()[0])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group1()[1]),
                (self.group3()[1] + other.group1()[2]),
                (self.group3()[2] + other.group1()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group1()[0])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group1()[1]),
                (self.group3()[1] + other.group1()[2]),
                (self.group3()[2] + other.group1()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            (self.group2() + other.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            (self.group2() + other.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[0], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() + other.group1()),
        );
    }
}
impl std::ops::AddAssign<AntiFlector> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlector) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() + other.group1()),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group1()[3],
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e31], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other.group1()[0] + self.group3()[0]),
                (other.group1()[1] + self.group3()[1]),
                (other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiMysteryDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other.group1()[0] + self.group3()[0]),
                (other.group1()[1] + self.group3()[1]),
                (other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            (self.group3() + other.group0()),
        );
    }
}
impl std::ops::AddAssign<AntiPlane> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            (self.group3() + other.group0()),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (other.group0()[0] + self.group3()[0]),
                (other.group0()[1] + self.group3()[1]),
                (other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiDipoleInversion {
    type Output = VersorEven;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e12345]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiSphereOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] + other.group0()[0]),
                (self.group3()[1] + other.group0()[1]),
                (self.group3()[2] + other.group0()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Circle> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Circle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for AntiDipoleInversion {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group2()[3],
            ]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group2()[3],
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
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[0] + self.group2()[3])]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Flector> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            other.group1()[3],
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[3],
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Horizon> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<Infinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e5])]),
        );
    }
}
impl std::ops::AddAssign<Infinity> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other[e5])]),
        );
    }
}
impl std::ops::Add<Line> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Line> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Line) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for AntiDipoleInversion {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: LineOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) + other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for AntiDipoleInversion {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       15        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]) + other.group1()),
            // e5
            (self.group3()[3] + other[e1]),
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            (self.group1() + other.group6()),
            // e423, e431, e412
            (self.group0() + other.group7()),
            // e235, e315, e125
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) + other.group8()),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
    }
}
impl std::ops::Add<MysteryCircle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for AntiDipoleInversion {
    fn add_assign(&mut self, other: MysteryCircle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e425]]),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: NullVersorEvenAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<Origin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::AddAssign<Origin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e4])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other[e2]]) + self.group3()),
        );
    }
}
impl std::ops::AddAssign<RoundPoint> for AntiDipoleInversion {
    fn add_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
            // e1, e2, e3, e5
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other[e2]]) + self.group3()),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
    }
}
impl std::ops::AddAssign<RoundPointAtOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: RoundPointAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (other.group0()[1] + self.group3()[3])]),
        );
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Sphere> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[1], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[0],
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       15        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) + other.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]) + other.group3()),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group2()[3] + other.group1()[3])]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) + other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] + other.group0()[1]),
                (self.group3()[1] + other.group0()[2]),
                (self.group3()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
    }
}
impl std::ops::AddAssign<VersorEvenAtOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: VersorEvenAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) + self.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] + other.group1()[3])]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group2()[3] + other.group1()[3])]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       12        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]) + self.group3()),
        );
    }
}
impl std::ops::AddAssign<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    fn add_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]) + self.group2()),
            // e1, e2, e3, e5
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]) + self.group3()),
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group1()[3],
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       35       51        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       20       33        0
    //  no simd       23       39        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       27       43        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       23       39        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        1        2        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       48       60        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd3        0        1        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       31       46        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       21        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       25        0
    //  no simd       23       37        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       25       43        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiLine> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiLineOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       11       27        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       19        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       31        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiPlane> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       24       40        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleAligningOrigin> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       13       19        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleAtOrigin> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        9        0
    //  no simd        7       12        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleOrthogonalOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleRotor> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       13       19        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<Dipole> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleAligningOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleInversion> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       21       35        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        1        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       27       38        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<DualNum> for AntiDipoleInversion {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<FlatPoint> for AntiDipoleInversion {
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
impl std::ops::BitXor<FlatPointAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<Flector> for AntiDipoleInversion {
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
impl std::ops::BitXor<FlectorAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<FlectorOnOrigin> for AntiDipoleInversion {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       10        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<Line> for AntiDipoleInversion {
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
impl std::ops::BitXor<LineAtInfinity> for AntiDipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiDipoleInversion {
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
impl std::ops::BitXor<MotorAtInfinity> for AntiDipoleInversion {
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
impl std::ops::BitXor<MotorOnOrigin> for AntiDipoleInversion {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       60        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       56       77        0
    //  no simd       89      120        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for AntiDipoleInversion {
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
impl std::ops::BitXor<MysteryDipoleInversion> for AntiDipoleInversion {
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
impl std::ops::BitXor<MysteryVersorEven> for AntiDipoleInversion {
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
impl std::ops::BitXor<MysteryVersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       21       34        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for AntiDipoleInversion {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<NullSphereAtOrigin> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<Origin> for AntiDipoleInversion {
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
impl std::ops::BitXor<Plane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       40        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiDipoleInversion {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        0
    //    simd3        1        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       48       60        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       26        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       22       38        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd3        0        1        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       31       46        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       25        0
    //  no simd       16       31        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiDipoleInversion {
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
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd3        1        2        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       42       54        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       31        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       31       47        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       33        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       37       53        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    fn from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
                anti_dipole_inversion_at_infinity[e321],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
                0.0,
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e1],
                anti_dipole_inversion_at_infinity[e2],
                anti_dipole_inversion_at_infinity[e3],
                anti_dipole_inversion_at_infinity[e5],
            ]),
        );
    }
}

impl From<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    fn from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([anti_dipole_inversion_on_origin[e1], anti_dipole_inversion_on_origin[e2], anti_dipole_inversion_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    fn from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([
                anti_dipole_inversion_orthogonal_origin[e423],
                anti_dipole_inversion_orthogonal_origin[e431],
                anti_dipole_inversion_orthogonal_origin[e412],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e415],
                anti_dipole_inversion_orthogonal_origin[e425],
                anti_dipole_inversion_orthogonal_origin[e435],
                0.0,
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                anti_dipole_inversion_orthogonal_origin[e4],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_orthogonal_origin[e5]]),
        );
    }
}

impl From<AntiDipoleOnOrigin> for AntiDipoleInversion {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for AntiDipoleInversion {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for AntiDipoleInversion {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for AntiDipoleInversion {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], anti_flector[e5]]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for AntiDipoleInversion {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    fn from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_mystery_dipole_inversion[e415],
                anti_mystery_dipole_inversion[e425],
                anti_mystery_dipole_inversion[e435],
                anti_mystery_dipole_inversion[e321],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_mystery_dipole_inversion[e1], anti_mystery_dipole_inversion[e2], anti_mystery_dipole_inversion[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for AntiDipoleInversion {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], anti_plane[e5]]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for AntiDipoleInversion {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiSphereOnOrigin> for AntiDipoleInversion {
    fn from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, anti_sphere_on_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], 0.0]),
        );
    }
}

impl From<Circle> for AntiDipoleInversion {
    fn from(circle: Circle) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle[e423], circle[e431], circle[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAligningOrigin> for AntiDipoleInversion {
    fn from(circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for AntiDipoleInversion {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtOrigin> for AntiDipoleInversion {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOnOrigin> for AntiDipoleInversion {
    fn from(circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for AntiDipoleInversion {
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for AntiDipoleInversion {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
        );
    }
}

impl From<Line> for AntiDipoleInversion {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for AntiDipoleInversion {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for AntiDipoleInversion {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for AntiDipoleInversion {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, motor_at_infinity[e5]]),
        );
    }
}

impl From<MysteryCircle> for AntiDipoleInversion {
    fn from(mystery_circle: MysteryCircle) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435], mystery_circle[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullCircleAtOrigin> for AntiDipoleInversion {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    fn from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_even_at_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Origin> for AntiDipoleInversion {
    fn from(origin: Origin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<RoundPoint> for AntiDipoleInversion {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e5]]),
        );
    }
}

impl From<RoundPointAtOrigin> for AntiDipoleInversion {
    fn from(round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
        );
    }
}

impl From<VersorEvenAtOrigin> for AntiDipoleInversion {
    fn from(versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e5]]),
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    fn from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e4],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e5],
            ]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       74       90        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       77        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       83       99        0
    //  no simd      149      165        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70       86        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      134      150        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       89      105        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       50       67        0
    //  no simd      104      121        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       89      105        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       42       42        0
    // Totals...
    // yes simd       83       99        0
    //  no simd      209      225        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       72        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       77       96        0
    //  no simd      149      168        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       55        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       53       72        0
    //  no simd      104      123        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       61       77        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       83       99        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       45       64        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       22        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       25        0
    //  no simd       14       34        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       30        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       48       60        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       39        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       46       60        0
    //  no simd      109      123        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4       12       14        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       48       60        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       74       90        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       29       45        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       53       69        0
    //  no simd      104      120        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       44       60        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       33        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       26       44        0
    //  no simd       59       77        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       53        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       89      105        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       48       63        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       29       45        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       30        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       25        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       17       34        0
    //  no simd       44       61        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       53       69        0
    //  no simd      104      120        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       74        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      134      150        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       75       91        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      119      135        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       64        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       56       75        0
    //  no simd       89      108        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       74       90        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       74        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       74       90        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       89      105        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      149      165        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       74        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      134      150        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       89      105        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       50       67        0
    //  no simd      104      121        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       89      105        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       66       82        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       83       99        0
    //  no simd      134      150        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       89      105        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       54        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       50       67        0
    //  no simd       89      106        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       62        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       74       90        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       85        0
    //    simd4       36       36        0
    // Totals...
    // yes simd      101      121        0
    //  no simd      209      229        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       29       29        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      164      180        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       50       66        0
    //  no simd      104      120        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       40        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       47       60        0
    //  no simd      107      120        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       81        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       23       34        0
    //  no simd       47       64        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       75       91        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      119      135        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       34        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       11       35        0
    //  no simd       14       38        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       30        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       29        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       23       37        0
    //  no simd       47       61        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiDipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       33       45        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       50       66        0
    //  no simd      104      120        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiDipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       48       60        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       20       31        0
    //  no simd       47       61        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiDipoleInversion {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       16        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiDipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       20        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       62       78        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       74       90        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiDipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       33        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       25       36        0
    //  no simd       34       45        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       29       45        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       45        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       53       64        0
    //  no simd      110      121        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4       11       12        0
    // Totals...
    // yes simd       19       24        0
    //  no simd       52       60        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       20       31        0
    //  no simd       47       61        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      146      178        0
    //    simd2        4        4        0
    //    simd3       46       46        0
    //    simd4       39       39        0
    // Totals...
    // yes simd      235      267        0
    //  no simd      448      480        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       24        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       17       34        0
    //  no simd       44       64        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       32        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       59       80        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       44       62        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4       14       14        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       89      105        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       25        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       35       49        0
    //  no simd      107      121        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4       22       23        0
    // Totals...
    // yes simd       38       54        0
    //  no simd      104      123        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       33       45        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       33       45        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       21       28        0
    //  no simd       51       61        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       23        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       33       45        0
    //  no simd       48       60        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       18        0
    //  no simd        3       24        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       31        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       39        0
    //  no simd       44       63        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       29       45        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       32        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       26       43        0
    //  no simd       59       76        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       30        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       32        0
    //  no simd       14       38        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiDipoleInversion {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       29        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       20       42        0
    //  no simd       59       81        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       26        0
    //  no simd       14       32        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       23        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       15       34        0
    //  no simd       48       67        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       44       44        0
    // Totals...
    // yes simd       92      108        0
    //  no simd      224      240        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       31       31        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      164      180        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       49        0
    //    simd4       33       33        0
    // Totals...
    // yes simd       68       82        0
    //  no simd      167      181        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       47       63        0
    //  no simd      104      120        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       59        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       75        0
    //  no simd      105      123        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       32       32        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      164      180        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       41       41        0
    // Totals...
    // yes simd      101      117        0
    //  no simd      224      240        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       30       30        0
    // Totals...
    // yes simd       74       90        0
    //  no simd      164      180        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      164      180        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn neg(self) -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e4
            (self.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e5
            (self.group3() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Not for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversion {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       15        0        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e4
            (-other.group2() + self.group2()),
            // e1, e2, e3, e5
            (-other.group3() + self.group3()),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversion> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e4
            (-other.group2() + self.group2()),
            // e1, e2, e3, e5
            (-other.group3() + self.group3()),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() - other.group2()),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() - other.group2()),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        8        3        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group1()[0])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group1()[1]),
                (self.group3()[1] - other.group1()[2]),
                (self.group3()[2] - other.group1()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleInversionOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group1()[0])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group1()[1]),
                (self.group3()[1] - other.group1()[2]),
                (self.group3()[2] - other.group1()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        6        3        0
    //  no simd       11        3        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            (self.group2() - other.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleInversionOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            (self.group2() - other.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[0] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() - other.group1()),
        );
    }
}
impl std::ops::SubAssign<AntiFlector> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlector) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group3() - other.group1()),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group1()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiDipoleInversion {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[e31] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group3()[0]),
                (-other.group1()[1] + self.group3()[1]),
                (-other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiMysteryDipoleInversion> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiMysteryDipoleInversion) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group3()[0]),
                (-other.group1()[1] + self.group3()[1]),
                (-other.group1()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            (self.group3() - other.group0()),
        );
    }
}
impl std::ops::SubAssign<AntiPlane> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiPlane) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            (self.group3() - other.group0()),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group3()[0]),
                (-other.group0()[1] + self.group3()[1]),
                (-other.group0()[2] + self.group3()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e12345] * -1.0)]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiSphereOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: AntiSphereOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group3()[0] - other.group0()[0]),
                (self.group3()[1] - other.group0()[1]),
                (self.group3()[2] - other.group0()[2]),
                self.group3()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Circle> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Circle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        3        0
    //  no simd        7        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        1        0
    //  no simd       10        1        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        1        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group2()[3] * -1.0),
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
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        1        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        1        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversion {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiDipoleInversion {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group2()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (other.group1() * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<DualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] * -1.0)]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[0] + self.group2()[3])]),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Flector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            (other.group1()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Horizon> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<Infinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e5])]),
        );
    }
}
impl std::ops::SubAssign<Infinity> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other[e5])]),
        );
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Line> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Line) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) - other.group1()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        1        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5        6        0
    //  no simd       15       17        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]) - other.group1()),
            // e5
            (self.group3()[3] - other[e1]),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() - other.group6()),
            // e423, e431, e412
            (self.group0() - other.group7()),
            // e235, e315, e125
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) - other.group8()),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
    }
}
impl std::ops::Sub<MysteryCircle> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: MysteryCircle) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e425] * -1.0)]),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            self.group2(),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other[e1234] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4        3        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<NullVersorEvenAtOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: NullVersorEvenAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<Origin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::SubAssign<Origin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e4])]),
            // e1, e2, e3, e5
            self.group3(),
        );
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            (other.group0()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        5        4        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            (Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other[e2] * -1.0)]) + self.group3()),
        );
    }
}
impl std::ops::SubAssign<RoundPoint> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: RoundPoint) {
        use crate::elements::*;
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
            // e1, e2, e3, e5
            (Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other[e2] * -1.0)]) + self.group3()),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
    }
}
impl std::ops::SubAssign<RoundPointAtOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: RoundPointAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (-other.group0()[0] + self.group2()[3])]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (-other.group0()[1] + self.group3()[3])]),
        );
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Sphere> for AntiDipoleInversion {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[1] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[0] * -1.0),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6        1        0
    //  no simd       15        1        0
    fn sub(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) - other.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]) - other.group3()),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        8        1        0
    //  no simd       11        1        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group2()[3] - other.group1()[3])]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        1        0
    //  no simd       11        1        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]) - other.group2()),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] - other.group0()[1]),
                (self.group3()[1] - other.group0()[2]),
                (self.group3()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        7        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        8        7        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group0()[3] * -1.0)]) + self.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
    }
}
impl std::ops::SubAssign<VersorEvenAtOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: VersorEvenAtOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e4
            (Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group0()[3] * -1.0)]) + self.group2()),
            // e1, e2, e3, e5
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group3()[3] - other.group1()[3])]),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], (self.group2()[3] - other.group1()[3])]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       11        0
    //  no simd       12       11        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            (Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group2()[3] * -1.0)]) + self.group2()),
            // e1, e2, e3, e5
            (Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), (other.group1()[3] * -1.0)]) + self.group3()),
        );
    }
}
impl std::ops::SubAssign<VersorEvenOrthogonalOrigin> for AntiDipoleInversion {
    fn sub_assign(&mut self, other: VersorEvenOrthogonalOrigin) {
        *self = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            (Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group2()[3] * -1.0)]) + self.group2()),
            // e1, e2, e3, e5
            (Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), (other.group1()[3] * -1.0)]) + self.group3()),
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversion {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            self.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group1(),
            // e423, e431, e412
            self.group0(),
            // e235, e315, e125
            Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group1()[3] * -1.0),
        );
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor: CircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_rotor[e423], circle_rotor[e431], circle_rotor[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_rotor_aligning_origin[e423], circle_rotor_aligning_origin[e431], circle_rotor_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_rotor_aligning_origin[e235], circle_rotor_aligning_origin[e315], circle_rotor_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e415],
                circle_rotor_aligning_origin_at_infinity[e425],
                circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e235],
                circle_rotor_aligning_origin_at_infinity[e315],
                circle_rotor_aligning_origin_at_infinity[e125],
                0.0,
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([circle_rotor_at_infinity[e235], circle_rotor_at_infinity[e315], circle_rotor_at_infinity[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_rotor_on_origin[e423], circle_rotor_on_origin[e431], circle_rotor_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for AntiDipoleInversion {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Motor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([motor[e235], motor[e315], motor[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, motor[e5]]),
        ));
    }
}

impl TryFrom<MotorOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(motor_on_origin: MotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversion {
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
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([multi_vector[e423], multi_vector[e431], multi_vector[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<MysteryCircleRotor> for AntiDipoleInversion {
    type Error = String;
    fn try_from(mystery_circle_rotor: MysteryCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryCircleRotor do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435], mystery_circle_rotor[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for AntiDipoleInversion {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435], mystery_versor_even[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([mystery_versor_even[e1], mystery_versor_even[e2], mystery_versor_even[e3], 0.0]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even[e423], versor_even[e431], versor_even[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e235, e315, e125, e4
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_aligning_origin[e423], versor_even_aligning_origin[e431], versor_even_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e4],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], versor_even_at_infinity[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversion {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversion { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e4]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}
