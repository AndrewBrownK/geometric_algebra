use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 469
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       4       0
//  Average:         6      11       0
//  Maximum:        96     123       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       5       0
//  Average:         9      15       0
//  Maximum:       192     224       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (self.group1() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: AntiCircleOnOrigin) {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (self.group1() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[3] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            (self.group1() + other.group1()),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[3] + other.group2()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            (self.group1() + other.group0()),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[3] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12
            (self.group1() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12
            (self.group1() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            (self.group1() + other.group0()),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ self.group0(), /* e23, e31, e12 */ (self.group1() + other.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        let addition = AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ self.group0(), /* e23, e31, e12 */ (self.group1() + other.group0()));
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: AntiMotorOnOrigin) {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12
            (self.group1() + Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
        *self = addition;
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] + other[e31])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiMysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiMysteryQuadNum) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] + other.group0()[1])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<AntiQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[2])]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[1]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[1])]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[1]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[1])]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] + other.group0()[0]), (self.group0()[1] + other.group0()[1]), (self.group0()[2] + other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] + other.group0()[0]), (self.group0()[1] + other.group0()[1]), (self.group0()[2] + other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            (self.group1() + other.group1()),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: Flector) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[e3215]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<Line> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[3] + other.group0()[0]), other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] + other.group3()[0]),
                (self.group0()[1] + other.group3()[1]),
                (self.group0()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            (self.group1() + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircle> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipole> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<MysteryQuadNum> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: NullDipoleAtOrigin) {
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[e1234]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: Plane) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<QuadNum> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<QuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<QuadNumAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<QuadNumOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<QuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e23, e31, e12
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for AntiCircleRotorOnOrigin {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
            // e23, e31, e12
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e4315]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() + other.group0()),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       31        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       16       32        0
    //  no simd       19       35        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       16       31        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        7       19        0
    //  no simd       10       25        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       13       29        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       19        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       23       39        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       17       32        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       26        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        7        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       22        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       11       26        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        9       13        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       18        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       14       26        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       13        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       14        0
    //  no simd        5       17        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       19        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       11        0
    fn bitxor(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       16        0
    //  no simd        5       19        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn bitxor(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn bitxor(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       12        0
    fn bitxor(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn bitxor(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn bitxor(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for AntiCircleRotorOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       12       20        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiCircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       16        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleAtInfinity> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleAtOrigin> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       10        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiCircleRotorOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       10        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       13       28        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<DipoleAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       10       22        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       18        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       33        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       27        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       11       20        0
    //  no simd       14       26        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       12       20        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd3        0        1        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       15       26        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       10       24        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<FlatPoint> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<FlatPointAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<FlectorAtInfinity> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<FlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for AntiCircleRotorOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<LineAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       12       20        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       13        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       43        0
    //    simd3        4        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       54        0
    //  no simd       48       80        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<MysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = MysteryCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       13        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = MysteryQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: MysteryQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       19        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       23        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       20        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullSphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for AntiCircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for AntiCircleRotorOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       13        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn bitxor(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn bitxor(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn bitxor(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       12        0
    fn bitxor(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       23        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       11        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiCircleRotorOnOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiCircleRotorOnOrigin {
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
impl std::ops::BitXor<SphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: SphereAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       24       40        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       15       27        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       21       33        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd        8       20        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       25        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       17       33        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       24       40        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       17       33        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       25        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       21       33        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    fn from(anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12]]),
        );
    }
}

impl From<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12]]),
        );
    }
}

impl From<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    fn from(anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor_on_origin[scalar]]),
            // e23, e31, e12
            Simd32x3::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12]]),
        );
    }
}

impl From<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}

impl From<Scalar> for AntiCircleRotorOnOrigin {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, scalar[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       33        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       49        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       41       56        0
    //  no simd       62       77        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       46        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       37       52        0
    //  no simd       55       70        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       34        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       25       40        0
    //  no simd       43       58        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       29       44        0
    //  no simd       50       65        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       32       40        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       89      105        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       43       59        0
    //  no simd       73       89        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       32        0
    //  no simd       36       44        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       53        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       43       59        0
    //  no simd       61       77        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       19        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       28        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       16       31        0
    //  no simd       25       40        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        9       11        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       53       72        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        6        6        0
    // Totals...
    // yes simd        6       14        0
    //  no simd       24       32        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       39        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       35       51        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       37        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       56       69        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       23       35        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       34        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       37       50        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       14        0
    fn mul(self, other: AntiMysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       31        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       21        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       12       28        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        6       25        0
    fn mul(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       21        0
    fn mul(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       24        0
    fn mul(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       11        0
    fn mul(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        6       21        0
    fn mul(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       19        0
    //  no simd       17       28        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       32        0
    //  no simd       36       44        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       50        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       54       70        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       51        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       47       63        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       38        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       42       58        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       38        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       42        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       33        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       37        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       27       40        0
    //  no simd       36       49        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       61       77        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       50        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       54       70        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       38        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       42       58        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       37        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       49       65        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       37        0
    //  no simd       32       40        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       42       58        0
    //  no simd       54       70        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       48        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       33       52        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       42        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       30       46        0
    //  no simd       42       58        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       42        0
    //  no simd       26       45        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       69        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       62       78        0
    //  no simd       89      105        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       63        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       50       69        0
    //  no simd       68       87        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       52       68        0
    //  no simd       73       89        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       25       42        0
    //  no simd       40       63        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       32        0
    //  no simd       36       44        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       52        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       41       59        0
    //  no simd       62       80        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       11       19        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       51        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       47       63        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       32        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       22       40        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       18       33        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       51        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       37       56        0
    //  no simd       52       71        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       13        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       13       21        0
    //  no simd       31       45        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       14        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       39        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       35       51        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       12       24        0
    //  no simd       18       30        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       52       68        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       77        0
    //    simd2        6        7        0
    //    simd3       21       23        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       96      123        0
    //  no simd      192      224        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       16       32        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       23       39        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       16       28        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       41        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       31       43        0
    //  no simd       37       49        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       14        0
    fn mul(self, other: MysteryQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       44       57        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       35       47        0
    //  no simd       44       56        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       16        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       16        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiCircleRotorOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       31        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       25        0
    //  no simd       12       28        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       15        0
    //  no simd        6       21        0
    fn mul(self, other: QuadNumAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       17        0
    fn mul(self, other: QuadNumAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       21        0
    //  no simd        9       24        0
    fn mul(self, other: QuadNumAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       11        0
    fn mul(self, other: QuadNumOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        6       21        0
    fn mul(self, other: QuadNumOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       20       35        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       14        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiCircleRotorOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd3        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       20       35        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       18        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       16        0
    //  no simd       17       25        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       57       73        0
    //  no simd       96      112        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       68       84        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       44       60        0
    //  no simd       80       96        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       28       44        0
    //  no simd       40       56        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       24       32        0
    //  no simd       36       44        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       68       85        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       66       82        0
    //  no simd       96      112        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       53       69        0
    //  no simd       80       96        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       55        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       45       63        0
    //  no simd       69       87        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiCircleRotorOnOrigin {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn neg(self) -> Self {
        let negation = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for AntiCircleRotorOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn not(self) -> Self::Output {
        let right_dual = CircleRotorOnOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
        return right_dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (self.group1() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleOnOrigin> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: AntiCircleOnOrigin) {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            (self.group1() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        7        4        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        7        3        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            (self.group1() - other.group1()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            (self.group1() - other.group0()),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[3] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[3] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12
            (self.group1() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiCircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12
            (self.group1() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        3        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            (self.group1() - other.group0()),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ self.group0(), /* e23, e31, e12 */ (self.group1() - other.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ self.group0(), /* e23, e31, e12 */ (self.group1() - other.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e23, e31, e12
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiMotorOnOrigin> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: AntiMotorOnOrigin) {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e23, e31, e12
            (self.group1() - Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        1        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] - other[e31])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiMysteryQuadNum) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] - other.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn sub(self, other: AntiQuadNumAligningOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[2])]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiQuadNumAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[1])]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn sub(self, other: AntiQuadNumAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[1] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiQuadNumOnOrigin) -> Self::Output {
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[1])]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiQuadNumOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[2] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        6        4        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] - other.group0()[0]), (self.group0()[1] - other.group0()[1]), (self.group0()[2] - other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6        3        0
    //  no simd        6        9        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        5        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group1()[3] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6        2        0
    //  no simd        6        5        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([(self.group0()[0] - other.group0()[0]), (self.group0()[1] - other.group0()[1]), (self.group0()[2] - other.group0()[2])]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        6        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (-other.group0() + Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            (self.group1() - other.group1()),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[e45] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e4235, e4315, e4125, e3215
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[e3215] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<Line> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiCircleRotorOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd3        1        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        7       25        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[3] - other.group0()[0]), (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from([
                (self.group0()[0] - other.group3()[0]),
                (self.group0()[1] - other.group3()[1]),
                (self.group0()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircle> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipole> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryQuadNum> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<NullDipoleAtOrigin> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: NullDipoleAtOrigin) {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group0()[3],
            ]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[e1234] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNum> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<QuadNumOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
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
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiCircleRotorOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e23, e31, e12
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for AntiCircleRotorOnOrigin {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
            // e23, e31, e12
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4315] * -1.0)]),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiCircleRotorOnOrigin {
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
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group1(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        7        9        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiCircleRotorOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        5        0
    //  no simd        4        8        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4        2        0
    //  no simd        7        5        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (self.group0() - other.group0()),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleRotor> for AntiCircleRotorOnOrigin {
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
        let el = anti_circle_rotor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_rotor[e41], anti_circle_rotor[e42], anti_circle_rotor[e43], anti_circle_rotor[scalar]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12]]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                anti_circle_rotor_aligning_origin[e41],
                anti_circle_rotor_aligning_origin[e42],
                anti_circle_rotor_aligning_origin[e43],
                anti_circle_rotor_aligning_origin[scalar],
            ]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor_aligning_origin[e23], anti_circle_rotor_aligning_origin[e31], anti_circle_rotor_aligning_origin[e12]]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_aligning_origin_at_infinity[scalar]]),
            // e23, e31, e12
            Simd32x3::from([
                anti_circle_rotor_aligning_origin_at_infinity[e23],
                anti_circle_rotor_aligning_origin_at_infinity[e31],
                anti_circle_rotor_aligning_origin_at_infinity[e12],
            ]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for AntiCircleRotorOnOrigin {
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
        let el = anti_circle_rotor_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_circle_rotor_at_infinity[scalar]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor_at_infinity[e23], anti_circle_rotor_at_infinity[e31], anti_circle_rotor_at_infinity[e12]]),
        ));
    }
}

impl TryFrom<AntiLine> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_line[e23], anti_line[e31], anti_line[e12]]),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[scalar]]),
            // e23, e31, e12
            Simd32x3::from([anti_motor[e23], anti_motor[e31], anti_motor[e12]]),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_circle_rotor[scalar]]),
            // e23, e31, e12
            Simd32x3::from([anti_mystery_circle_rotor[e23], anti_mystery_circle_rotor[e31], anti_mystery_circle_rotor[e12]]),
        ));
    }
}

impl TryFrom<AntiMysteryQuadNum> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_mystery_quad_num: AntiMysteryQuadNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_quad_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryQuadNum do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_mystery_quad_num[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNum> for AntiCircleRotorOnOrigin {
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
        let el = anti_quad_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_quad_num[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNum do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNumAligningOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_quad_num_aligning_origin: AntiQuadNumAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_quad_num_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNumAligningOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNumAligningOriginAtInfinity> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_quad_num_aligning_origin_at_infinity: AntiQuadNumAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num_aligning_origin_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNumAligningOriginAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_aligning_origin_at_infinity[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNumAtInfinity> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_quad_num_at_infinity: AntiQuadNumAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_quad_num_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNumAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_at_infinity[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiQuadNumOnOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_quad_num_on_origin: AntiQuadNumOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_quad_num_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiQuadNumOnOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_quad_num_on_origin[scalar]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                anti_versor_even_on_origin[e41],
                anti_versor_even_on_origin[e42],
                anti_versor_even_on_origin[e43],
                anti_versor_even_on_origin[scalar],
            ]),
            // e23, e31, e12
            Simd32x3::from([anti_versor_even_on_origin[e23], anti_versor_even_on_origin[e31], anti_versor_even_on_origin[e12]]),
        ));
    }
}

impl TryFrom<Dipole> for AntiCircleRotorOnOrigin {
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
        let el = dipole[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from([dipole[e23], dipole[e31], dipole[e12]]),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for AntiCircleRotorOnOrigin {
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
        let el = dipole_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for AntiCircleRotorOnOrigin {
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
        let el = dipole_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12]]),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversion> for AntiCircleRotorOnOrigin {
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
        let el = dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12]]),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for AntiCircleRotorOnOrigin {
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
        let el = dipole_inversion_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_aligning_origin[e41], dipole_inversion_aligning_origin[e42], dipole_inversion_aligning_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for AntiCircleRotorOnOrigin {
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
        let el = dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([dipole_inversion_at_infinity[e23], dipole_inversion_at_infinity[e31], dipole_inversion_at_infinity[e12]]),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_at_origin[e41], dipole_inversion_at_origin[e42], dipole_inversion_at_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionOnOrigin> for AntiCircleRotorOnOrigin {
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
        let el = dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from DipoleInversionOnOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_inversion_on_origin[e41], dipole_inversion_on_origin[e42], dipole_inversion_on_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                dipole_inversion_orthogonal_origin[e41],
                dipole_inversion_orthogonal_origin[e42],
                dipole_inversion_orthogonal_origin[e43],
                0.0,
            ]),
            // e23, e31, e12
            Simd32x3::from([
                dipole_inversion_orthogonal_origin[e23],
                dipole_inversion_orthogonal_origin[e31],
                dipole_inversion_orthogonal_origin[e12],
            ]),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from DipoleOnOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12]]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from MultiVector do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12
            Simd32x3::from([multi_vector[e23], multi_vector[e31], multi_vector[e12]]),
        ));
    }
}

impl TryFrom<MysteryDipole> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from MysteryDipole do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([mystery_dipole[e23], mystery_dipole[e31], mystery_dipole[e12]]),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from MysteryDipoleInversion do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12
            Simd32x3::from([mystery_dipole_inversion[e23], mystery_dipole_inversion[e31], mystery_dipole_inversion[e12]]),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for AntiCircleRotorOnOrigin {
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
            let mut error = "Elements from MysteryVersorOdd do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_odd[scalar]]),
            // e23, e31, e12
            Simd32x3::from([mystery_versor_odd[e23], mystery_versor_odd[e31], mystery_versor_odd[e12]]),
        ));
    }
}

impl TryFrom<NullDipoleInversionAtOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(null_dipole_inversion_at_origin: NullDipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullDipoleInversionAtOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_dipole_inversion_at_origin[e41], null_dipole_inversion_at_origin[e42], null_dipole_inversion_at_origin[e43], 0.0]),
            // e23, e31, e12
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiCircleRotorOnOrigin {
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
        let el = versor_odd[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
        let el = versor_odd[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[scalar]]),
            // e23, e31, e12
            Simd32x3::from([versor_odd[e23], versor_odd[e31], versor_odd[e12]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        let el = versor_odd_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[scalar]]),
            // e23, e31, e12
            Simd32x3::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12]]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for AntiCircleRotorOnOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into AntiCircleRotorOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                versor_odd_orthogonal_origin[e41],
                versor_odd_orthogonal_origin[e42],
                versor_odd_orthogonal_origin[e43],
                versor_odd_orthogonal_origin[scalar],
            ]),
            // e23, e31, e12
            Simd32x3::from([versor_odd_orthogonal_origin[e23], versor_odd_orthogonal_origin[e31], versor_odd_orthogonal_origin[e12]]),
        ));
    }
}