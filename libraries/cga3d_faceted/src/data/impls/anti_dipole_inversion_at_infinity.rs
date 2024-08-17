use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 386
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:         9      14       0
//  Maximum:       162     191       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        17      22       0
//  Maximum:       320     353       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            (self.group0() + other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group2() + other.group3()),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() + other.group0()),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            (self.group2() + other.group2()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() + other.group0()),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            (self.group2() + other.group2()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group1()[0]]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[1]),
                (self.group2()[1] + other.group1()[2]),
                (self.group2()[2] + other.group1()[3]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e321])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e321])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            (self.group2() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlector) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            (self.group2() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[1]),
                (self.group2()[1] + other.group0()[2]),
                (self.group2()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiFlectorOnOrigin) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[1]),
                (self.group2()[1] + other.group0()[2]),
                (self.group2()[2] + other.group0()[3]),
                self.group2()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotor> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (self.group2() + other.group0()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiPlane) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (self.group2() + other.group0()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
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
impl std::ops::AddAssign<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
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
impl std::ops::Add<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e1, e2, e3, e5
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
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: Circle) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            (self.group0() + other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() + other.group0()),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() + other.group0()),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e415, e425, e435, e321
            (self.group0() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group1()[3], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group1()[3], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            (self.group0() + other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[1], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group2()[3] + other.group0()[0])]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Infinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e5])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Line> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: Line) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            (self.group1() + other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() + other.group0()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: LineAtInfinity) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() + other.group0()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: LineOnOrigin) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[3], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            (other.group1() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn add_assign(&mut self, other: MotorAtInfinity) {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[3], self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd       11        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
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
            (self.group0() + other.group6()),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            (self.group1() + other.group8()),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[e4]]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e1, e2, e3, e5
            (self.group2() + Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other[e2]])),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[1])]),
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[1], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            (self.group0() + other.group1()),
            // e235, e315, e125, e5
            (other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2()[0] + other.group3()[0]),
                (self.group2()[1] + other.group3()[1]),
                (self.group2()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            (other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                other.group0()[0],
                (self.group2()[0] + other.group0()[1]),
                (self.group2()[1] + other.group0()[2]),
                (self.group2()[2] + other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            (self.group0() + other.group1()),
            // e235, e315, e125, e5
            (other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group0()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e5
            (self.group2() + Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       34        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       27       42        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       34        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       23       38        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       26        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       21       30        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       17       32        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        3        5        0
    // Totals...
    // yes simd        6       16        0
    //  no simd       15       31        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiDipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       15        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       26        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       20        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       21       33        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       19        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       13        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       19        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<Dipole> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       22        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleInversion> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        8       22        0
    //  no simd       14       28        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<FlatPoint> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       10        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<Line> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       54        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       42       66        0
    //  no simd       62       96        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<Origin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<Plane> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiDipoleInversionAtInfinity {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       15       31        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        9       28        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::BitXor<VersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       30        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       34       46        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       33        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       27       39        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125
            Simd32x3::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125
            Simd32x3::from([anti_flector[e235], anti_flector[e315], anti_flector[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], anti_flector[e5]]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], anti_plane[e5]]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
        );
    }
}

impl From<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125
            Simd32x3::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for AntiDipoleInversionAtInfinity {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
        );
    }
}

impl From<Line> for AntiDipoleInversionAtInfinity {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from([line[e235], line[e315], line[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, motor_at_infinity[e5]]),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       62       78        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       73        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       94      110        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       33        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       29       41        0
    //  no simd       53       65        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4       14       15        0
    // Totals...
    // yes simd       22       35        0
    //  no simd       64       80        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       53        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       46       62        0
    //  no simd       73       89        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       81        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       44        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       46       60        0
    //  no simd       94      108        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       52       68        0
    //  no simd       88      104        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       60       76        0
    //  no simd      105      121        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       28       48        0
    //  no simd       40       60        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        7       18        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum> for AntiDipoleInversionAtInfinity {
    fn mul_assign(&mut self, other: AntiDualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       22        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       36        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       44        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       37       52        0
    //  no simd       61       76        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       12       20        0
    //  no simd       36       44        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       34        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       27       39        0
    //  no simd       42       54        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       24       36        0
    //  no simd       60       72        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       44        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       28       47        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       33        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       22        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       17       35        0
    //  no simd       32       56        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       49       65        0
    //  no simd       88      104        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       58        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       94      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       67        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       83       99        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       21        0
    //    simd4       10       12        0
    // Totals...
    // yes simd       27       33        0
    //  no simd       57       69        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       50        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       50       66        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       56       72        0
    //  no simd       62       78        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       61       77        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       60       76        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       64       80        0
    //  no simd       94      110        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       35       47        0
    //  no simd       53       65        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       32       40        0
    //  no simd       68       76        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       65        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       73       89        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       58        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       55       71        0
    //  no simd       94      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       33        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       61       77        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4       11       13        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       53       69        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       46        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       35       51        0
    //  no simd       50       66        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       50       66        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4       22       24        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       93      109        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       33       50        0
    //  no simd       72       89        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       36        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       37       53        0
    //  no simd       88      104        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       73        0
    //  no simd      105      121        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       19       33        0
    //  no simd       40       60        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       51       67        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       59       75        0
    //  no simd       83       99        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        2        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        7       22        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       22        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        6        8        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       24       39        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        5        9        0
    //  no simd       17       21        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       24       39        0
    //  no simd       60       75        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        7        0
    // no simd       24       28        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       32       47        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiDipoleInversionAtInfinity {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       38        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       42       54        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        5       12        0
    //  no simd       17       24        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       33        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       37       43        0
    //  no simd       67       73        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        6        7        0
    // no simd       24       28        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       14       21        0
    //  no simd       35       45        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       94      121        0
    //    simd2        4        4        0
    //    simd3       38       40        0
    //    simd4       26       26        0
    // Totals...
    // yes simd      162      191        0
    //  no simd      320      353        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       33       45        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       42        0
    //  no simd       33       45        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::Mul<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::Mul<Origin> for AntiDipoleInversionAtInfinity {
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
impl std::ops::Mul<Plane> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       28       43        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       35        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       24       42        0
    //  no simd       39       63        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       20        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       23        0
    //  no simd        6       32        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiDipoleInversionAtInfinity {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       27        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       42       55        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       15        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        6       31        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       20        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       35       52        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       91      107        0
    //  no simd      160      176        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       32        0
    //    simd4       20       22        0
    // Totals...
    // yes simd       48       54        0
    //  no simd      108      120        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       72       88        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       52       68        0
    //  no simd       88      104        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       65       81        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       49        0
    //    simd4       32       32        0
    // Totals...
    // yes simd       64       81        0
    //  no simd      160      177        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4       26       30        0
    // Totals...
    // yes simd       26       34        0
    //  no simd      104      124        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       59       75        0
    //  no simd      116      132        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Not for AntiDipoleInversionAtInfinity {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn not(self) -> Self::Output {
        let dual = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] * -1.0)]),
        );
        return dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        2        0
    //  no simd       11        4        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group0() - other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e5
            (self.group2() - other.group3()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() - other.group0()),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            (self.group2() - other.group2()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiDipoleInversionAtInfinity) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() - other.group0()),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            (self.group2() - other.group2()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        4        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group1()[0] * -1.0)]),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[1]),
                (self.group2()[1] - other.group1()[2]),
                (self.group2()[2] - other.group1()[3]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        4        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        3        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e321])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e321])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            (self.group2() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlector> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlector) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            (self.group2() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[1]),
                (self.group2()[1] - other.group0()[2]),
                (self.group2()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiFlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiFlectorOnOrigin) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[1]),
                (self.group2()[1] - other.group0()[2]),
                (self.group2()[2] - other.group0()[3]),
                self.group2()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (self.group2() - other.group0()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlane> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiPlane) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            (self.group2() - other.group0()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
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
impl std::ops::SubAssign<AntiPlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
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
impl std::ops::Sub<AntiScalar> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other[e12345] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e5
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
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        3        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group0() - other.group1()),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        3        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() - other.group0()),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (self.group0() - other.group0()),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        3        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                0.0,
            ]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        4        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            (self.group0() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        1        0
    //  no simd        6        4        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        1        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group1()[3] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group1()[3] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            (self.group0() - other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group2()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        1        0
    //  no simd        3        4        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (other.group1() * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[1] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group2()[3] - other.group0()[0])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other[e3215] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Infinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Infinity) {
        use crate::elements::*;
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e5])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Line> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: Line) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            (self.group1() - other.group1()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() - other.group0()),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() - other.group0()),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<LineOnOrigin> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125
            self.group1(),
            // e1, e2, e3, e5
            self.group2(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[3] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            (-other.group1() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<MotorAtInfinity> for AntiDipoleInversionAtInfinity {
    fn sub_assign(&mut self, other: MotorAtInfinity) {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[3] * -1.0), self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        2        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6        8        0
    //  no simd       11       21        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
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
            (self.group0() - other.group6()),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() - other.group8()),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other[e1234] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[e4] * -1.0)]),
            // e1, e2, e3, e5
            self.group2(),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        4        1        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e5
            (self.group2() - Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other[e2]])),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[1] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5        2        0
    //  no simd       11        5        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            (self.group0() - other.group1()),
            // e235, e315, e125, e5
            (-other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group2()[0] - other.group3()[0]),
                (self.group2()[1] - other.group3()[1]),
                (self.group2()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        7        5        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            (-other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        1        0
    //  no simd       11        1        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (self.group2()[0] - other.group0()[1]),
                (self.group2()[1] - other.group0()[2]),
                (self.group2()[2] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            (self.group0() - other.group1()),
            // e235, e315, e125, e5
            (-other.group2() + Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]])),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        4        4        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group1()[3])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                self.group0()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        1        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        2        0
    //  no simd        8        4        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e5
            (self.group2() - Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[0] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e5
            self.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            self.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            self.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}

impl TryFrom<AntiDipoleInversion> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435], anti_dipole_inversion[e321]]),
            // e235, e315, e125
            Simd32x3::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([anti_dipole_inversion[e1], anti_dipole_inversion[e2], anti_dipole_inversion[e3], anti_dipole_inversion[e5]]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_dipole_inversion_on_origin[e1], anti_dipole_inversion_on_origin[e2], anti_dipole_inversion_on_origin[e3], 0.0]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e415],
                anti_dipole_inversion_orthogonal_origin[e425],
                anti_dipole_inversion_orthogonal_origin[e435],
                0.0,
            ]),
            // e235, e315, e125
            Simd32x3::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_orthogonal_origin[e5]]),
        ));
    }
}

impl TryFrom<AntiDipoleOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiSphereOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from AntiSphereOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], 0.0]),
        ));
    }
}

impl TryFrom<Circle> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from Circle do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125
            Simd32x3::from([circle[e235], circle[e315], circle[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotor> for AntiDipoleInversionAtInfinity {
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
        let el = circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e321]]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOrigin> for AntiDipoleInversionAtInfinity {
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
        let el = circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor_aligning_origin[e235], circle_rotor_aligning_origin[e315], circle_rotor_aligning_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAligningOriginAtInfinity> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleRotorAligningOriginAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e415],
                circle_rotor_aligning_origin_at_infinity[e425],
                circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e235, e315, e125
            Simd32x3::from([
                circle_rotor_aligning_origin_at_infinity[e235],
                circle_rotor_aligning_origin_at_infinity[e315],
                circle_rotor_aligning_origin_at_infinity[e125],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorAtInfinity> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from CircleRotorAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e235, e315, e125
            Simd32x3::from([circle_rotor_at_infinity[e235], circle_rotor_at_infinity[e315], circle_rotor_at_infinity[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleRotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
        let el = circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotorOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from DualNum do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
        ));
    }
}

impl TryFrom<Motor> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from Motor do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from([motor[e235], motor[e315], motor[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, motor[e5]]),
        ));
    }
}

impl TryFrom<MotorOnOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from MotorOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from MultiVector do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125
            Simd32x3::from([multi_vector[e235], multi_vector[e315], multi_vector[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<RoundPoint> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from RoundPoint do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e5]]),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from RoundPointAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiDipoleInversionAtInfinity {
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
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from VersorEven do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e235, e315, e125
            Simd32x3::from([versor_even[e235], versor_even[e315], versor_even[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for AntiDipoleInversionAtInfinity {
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
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from([versor_even_aligning_origin[e235], versor_even_aligning_origin[e315], versor_even_aligning_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e235, e315, e125
            Simd32x3::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], versor_even_at_infinity[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from VersorEvenAtOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for AntiDipoleInversionAtInfinity {
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
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
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
            let mut error = "Elements from VersorEvenOnOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for AntiDipoleInversionAtInfinity {
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
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into AntiDipoleInversionAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125
            Simd32x3::from([versor_even_orthogonal_origin[e235], versor_even_orthogonal_origin[e315], versor_even_orthogonal_origin[e125]]),
            // e1, e2, e3, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e5],
            ]),
        ));
    }
}
