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
// Total Implementations: 393
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       2       0
//  Average:        10      14       0
//  Maximum:       170     202       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       4       0
//  Average:        16      21       0
//  Maximum:       320     352       0
impl std::ops::Add<AntiCircleOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiCircleRotor> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiCircleRotorAligningOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiCircleRotorAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiCircleRotorOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiDipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group0()[0]),
                (other.group0()[1] + self.group0()[1]),
                (other.group0()[2] + self.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                other.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        0        0
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group2()[3],
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
                other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3]]),
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        4        0        0
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiDipoleOnOrigin> for CircleRotor {
    fn add_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiDualNum> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], self.group2()[3]]),
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
impl std::ops::Add<AntiFlatOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiFlatOrigin> for CircleRotor {
    fn add_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e321])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiFlatPoint> for CircleRotor {
    fn add_assign(&mut self, other: AntiFlatPoint) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiFlector> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                other.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Add<AntiLine> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiLineOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiMotor> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiMotorOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiMysteryCircleRotor> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[e31], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<AntiMysteryDipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
    }
}
impl std::ops::Add<AntiPlane> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: AntiPlane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
    }
}
impl std::ops::Add<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e12345])]),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for CircleRotor {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e12345])]),
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<Circle> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn add(self, other: Circle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<Circle> for CircleRotor {
    fn add_assign(&mut self, other: Circle) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd        9        0        0
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<CircleAligningOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleAligningOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[0] + self.group2()[0]),
                (other.group2()[1] + self.group2()[1]),
                (other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<CircleAtInfinity> for CircleRotor {
    fn add_assign(&mut self, other: CircleAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<CircleAtOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleAtOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<CircleOnOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<CircleOrthogonalOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<CircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e12345
            (other.group2() + self.group2()),
        );
    }
}
impl std::ops::AddAssign<CircleRotor> for CircleRotor {
    fn add_assign(&mut self, other: CircleRotor) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (other.group0() + self.group0()),
            // e415, e425, e435, e321
            (other.group1() + self.group1()),
            // e235, e315, e125, e12345
            (other.group2() + self.group2()),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() + other.group2()),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleRotorAligningOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() + other.group2()),
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() + other.group1()),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    fn add_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() + other.group1()),
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            (self.group2() + other.group1()),
        );
    }
}
impl std::ops::AddAssign<CircleRotorAtInfinity> for CircleRotor {
    fn add_assign(&mut self, other: CircleRotorAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            (self.group2() + other.group1()),
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        7        0        0
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::AddAssign<CircleRotorOnOrigin> for CircleRotor {
    fn add_assign(&mut self, other: CircleRotorOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[0] + self.group1()[0]),
                (other.group1()[1] + self.group1()[1]),
                (other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::Add<Dipole> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleAligningOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversion> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversionAligningOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversionAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversionAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversionOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DipoleOrthogonalOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] + self.group2()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
    }
}
impl std::ops::Add<FlatOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<FlatPoint> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<FlatPointAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<Flector> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<FlectorAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<FlectorOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<Horizon> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<Infinity> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<Line> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<Line> for CircleRotor {
    fn add_assign(&mut self, other: Line) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[0] + self.group2()[0]),
                (other.group1()[1] + self.group2()[1]),
                (other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<LineAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<LineAtInfinity> for CircleRotor {
    fn add_assign(&mut self, other: LineAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] + self.group2()[0]),
                (other.group0()[1] + self.group2()[1]),
                (other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<LineOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<LineOnOrigin> for CircleRotor {
    fn add_assign(&mut self, other: LineOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] + self.group1()[0]),
                (other.group0()[1] + self.group1()[1]),
                (other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] + other.group0()[3])]),
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
                other.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group0()[0]),
                (self.group2()[1] + other.group0()[1]),
                (self.group2()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::AddAssign<MotorOnOrigin> for CircleRotor {
    fn add_assign(&mut self, other: MotorOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group0()[0]),
                (self.group1()[1] + other.group0()[1]),
                (self.group1()[2] + other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other.group0()[3])]),
        );
    }
}
impl std::ops::Add<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd3        2        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd       11        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], (other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
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
impl std::ops::Add<MysteryCircle> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryCircle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<MysteryCircle> for CircleRotor {
    fn add_assign(&mut self, other: MysteryCircle) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e425])]),
        );
    }
}
impl std::ops::AddAssign<MysteryCircleRotor> for CircleRotor {
    fn add_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() + other.group0()),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] + other[e425])]),
        );
    }
}
impl std::ops::Add<MysteryDipole> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: MysteryDipole) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<MysteryDipoleInversion> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<MysteryVersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] + other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<NullCircleAtOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::AddAssign<NullCircleAtOrigin> for CircleRotor {
    fn add_assign(&mut self, other: NullCircleAtOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() + other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<NullDipoleInversionAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<NullSphereAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<NullVersorEvenAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
        );
    }
}
impl std::ops::Add<Origin> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
        );
    }
}
impl std::ops::Add<Plane> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<PlaneOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<RoundPoint> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other[e2]]),
            // e1, e2, e3, e4
            other.group0(),
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for CircleRotor {
    type Output = VersorEven;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
    }
}
impl std::ops::Add<Scalar> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self.group2()[3]]),
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
impl std::ops::Add<Sphere> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<SphereAtOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<SphereOnOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Add<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) + other.group0()),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e4
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) + other.group0()),
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
                other.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] + other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() + other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group2()[0]),
                (self.group2()[1] + other.group2()[1]),
                (self.group2()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) + other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] + other.group1()[0]),
                (self.group2()[1] + other.group1()[1]),
                (self.group2()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e1, e2, e3, e4
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOdd> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<VersorOddAtInfinity> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Add<VersorOddOrthogonalOrigin> for CircleRotor {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::BitXor<AntiCircleOnOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotor> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiCircleRotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOrigin> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAligningOriginAtInfinity> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAligningOriginAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       18        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorAtInfinity> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiCircleRotorAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiCircleRotorOnOrigin> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiCircleRotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for CircleRotor {
    type Output = CircleRotor;
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
impl std::ops::BitXorAssign<AntiDualNum> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        7       15        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotor> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiMotor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        9        0
    //  no simd        3       14        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       15        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMysteryCircleRotor> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiMysteryCircleRotor) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiVersorEvenOnOrigin> for CircleRotor {
    fn bitxor_assign(&mut self, other: AntiVersorEvenOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8        9        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for CircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        7       15        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       15        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MysteryVersorOdd> for CircleRotor {
    fn bitxor_assign(&mut self, other: MysteryVersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for CircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for CircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for CircleRotor {
    type Output = CircleRotor;
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
impl std::ops::BitXorAssign<Scalar> for CircleRotor {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        9        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for CircleRotor {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for CircleRotor {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       11        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7       13        0
    //  no simd        7       18        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddAtInfinity> for CircleRotor {
    fn bitxor_assign(&mut self, other: VersorOddAtInfinity) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOddOrthogonalOrigin> for CircleRotor {
    fn bitxor_assign(&mut self, other: VersorOddOrthogonalOrigin) {
        *self = self.wedge(other);
    }
}

impl From<AntiDipoleOnOrigin> for CircleRotor {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for CircleRotor {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for CircleRotor {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        );
    }
}

impl From<AntiScalar> for CircleRotor {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, anti_scalar[e12345]]),
        );
    }
}

impl From<Circle> for CircleRotor {
    fn from(circle: Circle) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle[e423], circle[e431], circle[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        );
    }
}

impl From<CircleAligningOrigin> for CircleRotor {
    fn from(circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
        );
    }
}

impl From<CircleAtInfinity> for CircleRotor {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
        );
    }
}

impl From<CircleAtOrigin> for CircleRotor {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
        );
    }
}

impl From<CircleOnOrigin> for CircleRotor {
    fn from(circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for CircleRotor {
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
        );
    }
}

impl From<CircleRotorAligningOrigin> for CircleRotor {
    fn from(circle_rotor_aligning_origin: CircleRotorAligningOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_rotor_aligning_origin[e423], circle_rotor_aligning_origin[e431], circle_rotor_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_aligning_origin[e415], circle_rotor_aligning_origin[e425], circle_rotor_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                circle_rotor_aligning_origin[e235],
                circle_rotor_aligning_origin[e315],
                circle_rotor_aligning_origin[e125],
                circle_rotor_aligning_origin[e12345],
            ]),
        );
    }
}

impl From<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    fn from(circle_rotor_aligning_origin_at_infinity: CircleRotorAligningOriginAtInfinity) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e415],
                circle_rotor_aligning_origin_at_infinity[e425],
                circle_rotor_aligning_origin_at_infinity[e435],
                0.0,
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                circle_rotor_aligning_origin_at_infinity[e235],
                circle_rotor_aligning_origin_at_infinity[e315],
                circle_rotor_aligning_origin_at_infinity[e125],
                circle_rotor_aligning_origin_at_infinity[e12345],
            ]),
        );
    }
}

impl From<CircleRotorAtInfinity> for CircleRotor {
    fn from(circle_rotor_at_infinity: CircleRotorAtInfinity) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                circle_rotor_at_infinity[e415],
                circle_rotor_at_infinity[e425],
                circle_rotor_at_infinity[e435],
                circle_rotor_at_infinity[e321],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                circle_rotor_at_infinity[e235],
                circle_rotor_at_infinity[e315],
                circle_rotor_at_infinity[e125],
                circle_rotor_at_infinity[e12345],
            ]),
        );
    }
}

impl From<CircleRotorOnOrigin> for CircleRotor {
    fn from(circle_rotor_on_origin: CircleRotorOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([circle_rotor_on_origin[e423], circle_rotor_on_origin[e431], circle_rotor_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_rotor_on_origin[e415], circle_rotor_on_origin[e425], circle_rotor_on_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, circle_rotor_on_origin[e12345]]),
        );
    }
}

impl From<Line> for CircleRotor {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
        );
    }
}

impl From<LineAtInfinity> for CircleRotor {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
        );
    }
}

impl From<LineOnOrigin> for CircleRotor {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorOnOrigin> for CircleRotor {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor_on_origin[e12345]]),
        );
    }
}

impl From<MysteryCircle> for CircleRotor {
    fn from(mystery_circle: MysteryCircle) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle[e415], mystery_circle[e425], mystery_circle[e435], mystery_circle[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}

impl From<MysteryCircleRotor> for CircleRotor {
    fn from(mystery_circle_rotor: MysteryCircleRotor) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_circle_rotor[e415], mystery_circle_rotor[e425], mystery_circle_rotor[e435], mystery_circle_rotor[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_circle_rotor[e12345]]),
        );
    }
}

impl From<NullCircleAtOrigin> for CircleRotor {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       50        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       38       54        0
    //  no simd       50       66        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       69        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       66       82        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       58       74        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       67       83        0
    //  no simd       94      110        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       53        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       43       59        0
    //  no simd       61       77        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       42       58        0
    //  no simd       72       88        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       45        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       38       53        0
    //  no simd       62       77        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       68       84        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       56        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       54       73        0
    //  no simd      105      124        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       47        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       40       58        0
    //  no simd       73       91        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       69       85        0
    //  no simd      105      121        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       34        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       17       38        0
    //  no simd       29       50        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       25        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       18        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       32       47        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       40        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       75       92        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       32        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       14       37        0
    //  no simd       29       52        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       50       66        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       18       33        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       48       64        0
    //  no simd       72       88        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       28       44        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       40       56        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       41        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       31       51        0
    //  no simd       61       81        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd3        1        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       35       46        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       18       36        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for CircleRotor {
    type Output = AntiCircleRotor;
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
impl std::ops::Mul<AntiSphereOnOrigin> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd3        2        3        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       24        0
    //  no simd       32       45        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       40       52        0
    //  no simd       76       88        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       62        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       58       74        0
    //  no simd       94      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       75        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       83       99        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       49        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       43       56        0
    //  no simd       64       77        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       50       66        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       54        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       50       66        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       61       77        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       53        0
    //    simd4       17       17        0
    // Totals...
    // yes simd       54       70        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       50       66        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       61       77        0
    //  no simd       94      110        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       57        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       43       63        0
    //  no simd       61       81        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       44        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       39       56        0
    //  no simd       75       92        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       46       62        0
    //  no simd       61       77        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       64       80        0
    //  no simd       94      110        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       51        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       40       58        0
    //  no simd       61       79        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       49        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       61       77        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       55        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       41       58        0
    //  no simd       50       67        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       80       96        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       62       78        0
    //  no simd      116      132        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd4       16       16        0
    // Totals...
    // yes simd       57       73        0
    //  no simd      105      121        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       39       55        0
    //  no simd       72       88        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       44        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       39       56        0
    //  no simd       75       92        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       57       73        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       69       85        0
    //  no simd      105      121        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       33        0
    //  no simd       32       48        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       75        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       65       81        0
    //  no simd       83       99        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       29        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       14        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       16        0
    //  no simd        0       22        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       18       32        0
    //  no simd       30       44        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for CircleRotor {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       26        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       28        0
    //  no simd       21       34        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       45        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       39       56        0
    //  no simd       72       89        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for CircleRotor {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       32       44        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       32        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       17       36        0
    //  no simd       29       48        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for CircleRotor {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       11        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for CircleRotor {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       17        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       46       62        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       47       63        0
    //  no simd       50       66        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for CircleRotor {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       52        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       45       62        0
    //  no simd       75       92        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for CircleRotor {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       17        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       35       45        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       35        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       38        0
    //  no simd       28       47        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      108      140        0
    //    simd2        1        1        0
    //    simd3       34       34        0
    //    simd4       27       27        0
    // Totals...
    // yes simd      170      202        0
    //  no simd      320      352        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       20        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       34       44        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       23        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       45       55        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       34        0
    //  no simd       28       46        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       41        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       37       50        0
    //  no simd       64       77        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       34        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       27       49        0
    //  no simd       72       94        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       13       14        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       72       88        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       21       36        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       21       36        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       23        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       35       47        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        1       12        0
    //  no simd        1       14        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       38        0
    //  no simd       32       44        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       14        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        1       16        0
    //  no simd        1       20        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       24        0
    //    simd3        1        2        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       12       31        0
    //  no simd       29       50        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       30        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       16       32        0
    //  no simd       18       36        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       21        0
    //    simd3        2        3        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       12       32        0
    //  no simd       40       62        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd3        0        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       18        0
    //  no simd        8       28        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for CircleRotor {
    type Output = CircleRotor;
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
impl std::ops::MulAssign<Scalar> for CircleRotor {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       18        0
    //    simd3        2        3        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       43       55        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       19        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       21        0
    //  no simd        8       26        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for CircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       22        0
    //    simd3        2        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       32       47        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4       27       27        0
    // Totals...
    // yes simd       79       95        0
    //  no simd      160      176        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for CircleRotor {
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
impl std::ops::Mul<VersorEvenAtInfinity> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       54        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       53       75        0
    //  no simd      116      138        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       47        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       39       58        0
    //  no simd       72       91        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       55        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       45       64        0
    //  no simd       72       91        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd4       19       19        0
    // Totals...
    // yes simd       59       75        0
    //  no simd      116      132        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       82       98        0
    //  no simd      160      176        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4       20       20        0
    // Totals...
    // yes simd       56       72        0
    //  no simd      116      132        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       71       87        0
    //  no simd      116      132        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for CircleRotor {
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn neg(self) -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e12345
            (self.group2() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Not for CircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiCircleRotor> for CircleRotor {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for CircleRotor {
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
            Simd32x2::from([(other.group2()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for CircleRotor {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiCircleRotorAtInfinity> for CircleRotor {
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
            Simd32x2::from([(other.group1()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiCircleRotorOnOrigin> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiDipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7        2        0
    //  no simd       10        5        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group0()[0]),
                (-other.group0()[1] + self.group0()[1]),
                (-other.group0()[2] + self.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                (other.group3()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        4        0
    //  no simd        7        4        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9        2        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group2()[3],
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
                (other.group0()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group2()[3] * -1.0)]),
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiDipoleOnOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: AntiDipoleOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDualNum> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), self.group2()[3]]),
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
impl std::ops::Sub<AntiFlatOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiFlatOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: AntiFlatOrigin) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e321])]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiFlatPoint> for CircleRotor {
    fn sub_assign(&mut self, other: AntiFlatPoint) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiFlector> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        4        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[0] + self.group1()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiLine> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiLineOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiMotor> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiMotorOnOrigin> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiMysteryCircleRotor> for CircleRotor {
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
            Simd32x2::from([(other[e31] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<AntiMysteryDipoleInversion> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        4        3        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e12345])]),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for CircleRotor {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e12345])]),
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<Circle> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn sub(self, other: Circle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<Circle> for CircleRotor {
    fn sub_assign(&mut self, other: Circle) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd        9        0        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<CircleAligningOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleAligningOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group2()[0] + self.group2()[0]),
                (-other.group2()[1] + self.group2()[1]),
                (-other.group2()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<CircleAtInfinity> for CircleRotor {
    fn sub_assign(&mut self, other: CircleAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (-other.group0() + self.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<CircleAtOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleAtOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        6        0        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<CircleOnOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        3        0
    //  no simd        7        3        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<CircleOrthogonalOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleOrthogonalOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (-other.group0()[3] + self.group1()[3])]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<CircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e12345
            (-other.group2() + self.group2()),
        );
    }
}
impl std::ops::SubAssign<CircleRotor> for CircleRotor {
    fn sub_assign(&mut self, other: CircleRotor) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (-other.group0() + self.group0()),
            // e415, e425, e435, e321
            (-other.group1() + self.group1()),
            // e235, e315, e125, e12345
            (-other.group2() + self.group2()),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() - other.group2()),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleRotorAligningOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() - other.group2()),
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        0        0
    //  no simd        7        0        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() - other.group1()),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAligningOriginAtInfinity> for CircleRotor {
    fn sub_assign(&mut self, other: CircleRotorAligningOriginAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            (self.group2() - other.group1()),
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            (self.group2() - other.group1()),
        );
    }
}
impl std::ops::SubAssign<CircleRotorAtInfinity> for CircleRotor {
    fn sub_assign(&mut self, other: CircleRotorAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            (self.group2() - other.group1()),
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        3        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        5        3        0
    //  no simd        7        3        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::SubAssign<CircleRotorOnOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: CircleRotorOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]) + self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group1()[0] + self.group1()[0]),
                (-other.group1()[1] + self.group1()[1]),
                (-other.group1()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::Sub<Dipole> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleAligningOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleAtInfinity> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleAtOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversion> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversionAligningOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversionAtInfinity> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversionAtOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversionOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DipoleOrthogonalOrigin> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (-other.group0()[1] + self.group2()[3])]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
    }
}
impl std::ops::Sub<FlatOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<FlatPoint> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<FlatPointAtInfinity> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<Flector> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<FlectorAtInfinity> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<FlectorOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<Horizon> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<Infinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other[e5] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<Line> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<Line> for CircleRotor {
    fn sub_assign(&mut self, other: Line) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group1()[0] + self.group2()[0]),
                (-other.group1()[1] + self.group2()[1]),
                (-other.group1()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<LineAtInfinity> for CircleRotor {
    fn sub_assign(&mut self, other: LineAtInfinity) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-other.group0()[0] + self.group2()[0]),
                (-other.group0()[1] + self.group2()[1]),
                (-other.group0()[2] + self.group2()[2]),
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<LineOnOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: LineOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-other.group0()[0] + self.group1()[0]),
                (-other.group0()[1] + self.group1()[1]),
                (-other.group0()[2] + self.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        0
    fn sub(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] - other.group0()[3])]),
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
                (other.group1()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group0()[0]),
                (self.group2()[1] - other.group0()[1]),
                (self.group2()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::SubAssign<MotorOnOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: MotorOnOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group0()[0]),
                (self.group1()[1] - other.group0()[1]),
                (self.group1()[2] - other.group0()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other.group0()[3])]),
        );
    }
}
impl std::ops::Sub<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        2        2        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        4        8        0
    //  no simd       11       21        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), (-other.group0()[1] + self.group2()[3])]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
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
impl std::ops::Sub<MysteryCircle> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<MysteryCircle> for CircleRotor {
    fn sub_assign(&mut self, other: MysteryCircle) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        5        0        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e425])]),
        );
    }
}
impl std::ops::SubAssign<MysteryCircleRotor> for CircleRotor {
    fn sub_assign(&mut self, other: MysteryCircleRotor) {
        use crate::elements::*;
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            self.group0(),
            // e415, e425, e435, e321
            (self.group1() - other.group0()),
            // e235, e315, e125, e12345
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group2()[3] - other[e425])]),
        );
    }
}
impl std::ops::Sub<MysteryDipole> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<MysteryDipoleInversion> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<MysteryVersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        5        3        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] - other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for CircleRotor {
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
            Simd32x2::from([(other.group0()[0] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<NullCircleAtOrigin> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::SubAssign<NullCircleAtOrigin> for CircleRotor {
    fn sub_assign(&mut self, other: NullCircleAtOrigin) {
        *self = CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() - other.group0()),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e12345
            self.group2(),
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<NullDipoleInversionAtOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<NullSphereAtOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<NullVersorEvenAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
        );
    }
}
impl std::ops::Sub<Origin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
        );
    }
}
impl std::ops::Sub<Plane> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<PlaneOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<RoundPoint> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other[e2] * -1.0)]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
    }
}
impl std::ops::Sub<Scalar> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), self.group2()[3]]),
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
impl std::ops::Sub<Sphere> for CircleRotor {
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
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<SphereAtOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<SphereOnOrigin> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group2()[3]]),
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
impl std::ops::Sub<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5        2        0
    //  no simd       11        5        0
    fn sub(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) - other.group0()),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        2        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        2        0
    //  no simd       10        2        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) - other.group0()),
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
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        4        0
    //  no simd        8        4        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group2()[3] - other.group0()[0])]),
            // e415, e425, e435, e321
            (self.group1() - other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group2()[0]),
                (self.group2()[1] - other.group2()[1]),
                (self.group2()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        2        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        7        1        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) - other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                self.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        7        2        0
    //  no simd        7        5        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                self.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[3])]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] - other.group1()[0]),
                (self.group2()[1] - other.group1()[1]),
                (self.group2()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
        );
    }
}
impl std::ops::Sub<VersorOdd> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<VersorOddAtInfinity> for CircleRotor {
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
            Simd32x2::from([(other.group0()[0] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
impl std::ops::Sub<VersorOddOrthogonalOrigin> for CircleRotor {
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
            Simd32x2::from([(other.group0()[3] * -1.0), self.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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

impl TryFrom<AntiDipoleInversion> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion[e423], anti_dipole_inversion[e431], anti_dipole_inversion[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([anti_dipole_inversion[e415], anti_dipole_inversion[e425], anti_dipole_inversion[e435], anti_dipole_inversion[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_dipole_inversion[e235], anti_dipole_inversion[e315], anti_dipole_inversion[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e415],
                anti_dipole_inversion_at_infinity[e425],
                anti_dipole_inversion_at_infinity[e435],
                anti_dipole_inversion_at_infinity[e321],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                anti_dipole_inversion_at_infinity[e235],
                anti_dipole_inversion_at_infinity[e315],
                anti_dipole_inversion_at_infinity[e125],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([anti_dipole_inversion_on_origin[e423], anti_dipole_inversion_on_origin[e431], anti_dipole_inversion_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_inversion_on_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDipoleInversionOrthogonalOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_dipole_inversion_orthogonal_origin: AntiDipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
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
            let mut error = "Elements from AntiDipoleInversionOrthogonalOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
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
            // e235, e315, e125, e12345
            Simd32x4::from([
                anti_dipole_inversion_orthogonal_origin[e235],
                anti_dipole_inversion_orthogonal_origin[e315],
                anti_dipole_inversion_orthogonal_origin[e125],
                0.0,
            ]),
        ));
    }
}

impl TryFrom<AntiFlector> for CircleRotor {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiFlectorOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlectorOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryDipoleInversion> for CircleRotor {
    type Error = String;
    fn try_from(anti_mystery_dipole_inversion: AntiMysteryDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryDipoleInversion do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                anti_mystery_dipole_inversion[e415],
                anti_mystery_dipole_inversion[e425],
                anti_mystery_dipole_inversion[e435],
                anti_mystery_dipole_inversion[e321],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for CircleRotor {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
        ));
    }
}

impl TryFrom<Motor> for CircleRotor {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e12345]]),
        ));
    }
}

impl TryFrom<MotorAtInfinity> for CircleRotor {
    type Error = String;
    fn try_from(motor_at_infinity: MotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MotorAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for CircleRotor {
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
            let mut error = "Elements from MultiVector do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([multi_vector[e423], multi_vector[e431], multi_vector[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e12345]]),
        ));
    }
}

impl TryFrom<MysteryVersorEven> for CircleRotor {
    type Error = String;
    fn try_from(mystery_versor_even: MysteryVersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorEven do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([mystery_versor_even[e415], mystery_versor_even[e425], mystery_versor_even[e435], mystery_versor_even[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, mystery_versor_even[e12345]]),
        ));
    }
}

impl TryFrom<NullVersorEvenAtOrigin> for CircleRotor {
    type Error = String;
    fn try_from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = null_versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from NullVersorEvenAtOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEven> for CircleRotor {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
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
            let mut error = "Elements from VersorEven do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even[e423], versor_even[e431], versor_even[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e12345]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_aligning_origin[e423], versor_even_aligning_origin[e431], versor_even_aligning_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e12345],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                versor_even_at_infinity[e235],
                versor_even_at_infinity[e315],
                versor_even_at_infinity[e125],
                versor_even_at_infinity[e12345],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e12345]]),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for CircleRotor {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
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
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into CircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e12345
            Simd32x4::from([versor_even_orthogonal_origin[e235], versor_even_orthogonal_origin[e315], versor_even_orthogonal_origin[e125], 0.0]),
        ));
    }
}
