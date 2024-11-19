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
// Total Implementations: 112
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       3       0
//  Average:        13      18       0
//  Maximum:       149     176       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         4       8       0
//  Average:        22      28       0
//  Maximum:       320     353       0
impl std::ops::Add<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::AddAssign<AntiCircleRotor> for AntiCircleRotor {
    fn add_assign(&mut self, other: AntiCircleRotor) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0() + self.group0(),
            // e23, e31, e12, e45
            other.group1() + self.group1(),
            // e15, e25, e35, scalar
            other.group2() + self.group2(),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiDualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1] + self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiLine> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                other.group0()[0] + self.group1()[0],
                other.group0()[1] + self.group1()[1],
                other.group0()[2] + self.group1()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other.group1()[0] + self.group2()[0],
                other.group1()[1] + self.group2()[1],
                other.group1()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<AntiLine> for AntiCircleRotor {
    fn add_assign(&mut self, other: AntiLine) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                other.group0()[0] + self.group1()[0],
                other.group0()[1] + self.group1()[1],
                other.group0()[2] + self.group1()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other.group1()[0] + self.group2()[0],
                other.group1()[1] + self.group2()[1],
                other.group1()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3] + other.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self.group1()[0] + other.group0()[0],
                self.group1()[1] + other.group0()[1],
                self.group1()[2] + other.group0()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0] + other.group1()[0], self.group2()[1] + other.group1()[1], self.group2()[2] + other.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Circle> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Dipole> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn add(self, other: Dipole) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other.group2()[0] + self.group2()[0],
                other.group2()[1] + self.group2()[1],
                other.group2()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<Dipole> for AntiCircleRotor {
    fn add_assign(&mut self, other: Dipole) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() + other.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                other.group2()[0] + self.group2()[0],
                other.group2()[1] + self.group2()[1],
                other.group2()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7        0        0
    //  no simd       10        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self.group0()[0] + other.group0()[0],
                self.group0()[1] + other.group0()[1],
                self.group0()[2] + other.group0()[2],
                self.group2()[3],
            ]),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] + other.group2()[0],
                self.group2()[1] + other.group2()[1],
                self.group2()[2] + other.group2()[2],
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<DualNum> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<FlatPoint> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] + other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self.group2()[0] + other.group0()[0],
                self.group2()[1] + other.group0()[1],
                self.group2()[2] + other.group0()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::AddAssign<FlatPoint> for AntiCircleRotor {
    fn add_assign(&mut self, other: FlatPoint) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] + other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self.group2()[0] + other.group0()[0],
                self.group2()[1] + other.group0()[1],
                self.group2()[2] + other.group0()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Add<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: Flector) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] + other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0] + other.group0()[0], self.group2()[1] + other.group0()[1], self.group2()[2] + other.group0()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
    }
}
impl std::ops::Add<Line> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Motor> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<MultiVector> for AntiCircleRotor {
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
            Simd32x2::from([other.group0()[0] + self.group2()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]) + other.group3(),
            // e41, e42, e43
            self.group0() + other.group4(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) + other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            other.group9(),
            // e1234
            other[e45],
        );
    }
}
impl std::ops::Add<Plane> for AntiCircleRotor {
    type Output = VersorOdd;
    fn add(self, other: Plane) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] + other[scalar]]),
        );
    }
}
impl std::ops::AddAssign<Scalar> for AntiCircleRotor {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] + other[scalar]]),
        );
    }
}
impl std::ops::Add<Sphere> for AntiCircleRotor {
    type Output = VersorOdd;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other[e4315]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<VersorEven> for AntiCircleRotor {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       11        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) + other.group0(),
            // e23, e31, e12, e45
            self.group1() + other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] + other.group2()[0],
                self.group2()[1] + other.group2()[1],
                self.group2()[2] + other.group2()[2],
                other.group2()[3],
            ]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       35        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       35       51        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       21       36        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       20       36        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       14       28        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for AntiCircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiCircleRotor {
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
impl std::ops::BitXor<CircleRotor> for AntiCircleRotor {
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
impl std::ops::BitXor<Dipole> for AntiCircleRotor {
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
impl std::ops::BitXor<DipoleInversion> for AntiCircleRotor {
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
impl std::ops::BitXor<DualNum> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiCircleRotor {
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
impl std::ops::BitXor<Flector> for AntiCircleRotor {
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
impl std::ops::BitXor<Line> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiCircleRotor {
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
impl std::ops::BitXor<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd3        7       10        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       48       68        0
    //  no simd       80      112        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       20       35        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiCircleRotor {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiCircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       40       56        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiLine> for AntiCircleRotor {
    fn from(from_anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([from_anti_line[e23], from_anti_line[e31], from_anti_line[e12], 0.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([from_anti_line[e15], from_anti_line[e25], from_anti_line[e35], 0.0]),
        );
    }
}

impl From<Dipole> for AntiCircleRotor {
    fn from(from_dipole: Dipole) -> Self {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([from_dipole[e41], from_dipole[e42], from_dipole[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([from_dipole[e23], from_dipole[e31], from_dipole[e12], from_dipole[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([from_dipole[e15], from_dipole[e25], from_dipole[e35], 0.0]),
        );
    }
}

impl From<FlatPoint> for AntiCircleRotor {
    fn from(from_flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, from_flat_point[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([from_flat_point[e15], from_flat_point[e25], from_flat_point[e35], 0.0]),
        );
    }
}

impl From<Scalar> for AntiCircleRotor {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, from_scalar[scalar]]),
        );
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       61        0
    //    simd4       15       15        0
    // Totals...
    // yes simd       60       76        0
    //  no simd      105      121        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       69        0
    //    simd4       24       24        0
    // Totals...
    // yes simd       77       93        0
    //  no simd      149      165        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       22        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       35        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       32       47        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       44        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       73       88        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       50       66        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       72       88        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       17        0
    //    simd3        1        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       29       47        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       70        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       64       80        0
    //  no simd       94      110        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       53       69        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       66       82        0
    //  no simd      105      121        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiCircleRotor {
    type Output = VersorOdd;
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
impl std::ops::Mul<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       65       81        0
    //    simd4       21       21        0
    // Totals...
    // yes simd       86      102        0
    //  no simd      149      165        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       26        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       39        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       29       47        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       51       67        0
    //  no simd       72       88        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       58        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       44       60        0
    //  no simd       50       66        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       52        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       45       61        0
    //  no simd       72       88        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73       97        0
    //    simd2       10       10        0
    //    simd3       37       40        0
    //    simd4       29       29        0
    // Totals...
    // yes simd      149      176        0
    //  no simd      320      353        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       29       44        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        2        3        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       40       56        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiCircleRotor {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        2        3        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       21       34        0
    //  no simd       40       55        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4       26       26        0
    // Totals...
    // yes simd       82       98        0
    //  no simd      160      176        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       68       84        0
    //    simd4       23       23        0
    // Totals...
    // yes simd       91      107        0
    //  no simd      160      176        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn neg(self) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group1() * Simd32x4::from(-1.0),
            // e15, e25, e35, scalar
            self.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd       11        0        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            -other.group0() + self.group0(),
            // e23, e31, e12, e45
            -other.group1() + self.group1(),
            // e15, e25, e35, scalar
            -other.group2() + self.group2(),
        );
    }
}
impl std::ops::SubAssign<AntiCircleRotor> for AntiCircleRotor {
    fn sub_assign(&mut self, other: AntiCircleRotor) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            -other.group0() + self.group0(),
            // e23, e31, e12, e45
            -other.group1() + self.group1(),
            // e15, e25, e35, scalar
            -other.group2() + self.group2(),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiCircleRotor {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0),
            // e5
            other.group3()[3] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        1        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], -other.group0()[1] + self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other.group0()[0] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0] * -1.0, other.group1()[1] * -1.0, other.group1()[2] * -1.0, 0.0]),
            // e5
            other.group1()[3] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * -1.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                -other.group0()[0] + self.group1()[0],
                -other.group0()[1] + self.group1()[1],
                -other.group0()[2] + self.group1()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                -other.group1()[0] + self.group2()[0],
                -other.group1()[1] + self.group2()[1],
                -other.group1()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<AntiLine> for AntiCircleRotor {
    fn sub_assign(&mut self, other: AntiLine) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                -other.group0()[0] + self.group1()[0],
                -other.group0()[1] + self.group1()[1],
                -other.group0()[2] + self.group1()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                -other.group1()[0] + self.group2()[0],
                -other.group1()[1] + self.group2()[1],
                -other.group1()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        1        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3] - other.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                self.group1()[0] - other.group0()[0],
                self.group1()[1] - other.group0()[1],
                self.group1()[2] - other.group0()[2],
                self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0] - other.group1()[0], self.group2()[1] - other.group1()[1], self.group2()[2] - other.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3] * -1.0]),
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * -1.0, other.group0()[1] * -1.0, other.group0()[2] * -1.0, 0.0]),
            // e5
            other.group0()[3] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other[e12345] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Circle> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiCircleRotor {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group2()[3] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Dipole> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       10        0        0
    fn sub(self, other: Dipole) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                -other.group2()[0] + self.group2()[0],
                -other.group2()[1] + self.group2()[1],
                -other.group2()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<Dipole> for AntiCircleRotor {
    fn sub_assign(&mut self, other: Dipole) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0() - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([
                -other.group2()[0] + self.group2()[0],
                -other.group2()[1] + self.group2()[1],
                -other.group2()[2] + self.group2()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7        2        0
    //  no simd       10        5        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                self.group0()[0] - other.group0()[0],
                self.group0()[1] - other.group0()[1],
                self.group0()[2] - other.group0()[2],
                self.group2()[3],
            ]),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] - other.group2()[0],
                self.group2()[1] - other.group2()[1],
                self.group2()[2] - other.group2()[2],
                other.group2()[3] * -1.0,
            ]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DualNum> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[1] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * -1.0]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<FlatPoint> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] - other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self.group2()[0] - other.group0()[0],
                self.group2()[1] - other.group0()[1],
                self.group2()[2] - other.group0()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::SubAssign<FlatPoint> for AntiCircleRotor {
    fn sub_assign(&mut self, other: FlatPoint) {
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] - other.group0()[3]]),
            // e15, e25, e35, scalar
            Simd32x4::from([
                self.group2()[0] - other.group0()[0],
                self.group2()[1] - other.group0()[1],
                self.group2()[2] - other.group0()[2],
                self.group2()[3],
            ]),
        );
    }
}
impl std::ops::Sub<Flector> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        1        0
    //  no simd        4        4        0
    fn sub(self, other: Flector) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] - other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0] - other.group0()[0], self.group2()[1] - other.group0()[1], self.group2()[2] - other.group0()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<Line> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * -1.0, other.group0()[1] * -1.0, other.group0()[2] * -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Motor> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[3] * -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * -1.0, other.group0()[1] * -1.0, other.group0()[2] * -1.0, 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiCircleRotor {
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
            Simd32x2::from([-other.group0()[0] + self.group2()[3], other.group0()[1] * -1.0]),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e1] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]) - other.group3(),
            // e41, e42, e43
            self.group0() - other.group4(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) - other.group5(),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            other.group9() * Simd32x4::from(-1.0),
            // e1234
            other[e45] * -1.0,
        );
    }
}
impl std::ops::Sub<Plane> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiCircleRotor {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], 0.0]),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e2] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] - other[scalar]]),
        );
    }
}
impl std::ops::SubAssign<Scalar> for AntiCircleRotor {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = AntiCircleRotor::from_groups(
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group2()[3] - other[scalar]]),
        );
    }
}
impl std::ops::Sub<Sphere> for AntiCircleRotor {
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
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], other[e4315] * -1.0]),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiCircleRotor {
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
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group2()[3], other.group0()[3] * -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other.group2()[3] * -1.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        5        2        0
    //  no simd       11        5        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]) - other.group0(),
            // e23, e31, e12, e45
            self.group1() - other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] - other.group2()[0],
                self.group2()[1] - other.group2()[1],
                self.group2()[2] - other.group2()[2],
                other.group2()[3] * -1.0,
            ]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}

impl TryFrom<AntiDualNum> for AntiCircleRotor {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[scalar]]),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiCircleRotor {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
            // e15, e25, e35, scalar
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], anti_motor[scalar]]),
        ));
    }
}

impl TryFrom<DipoleInversion> for AntiCircleRotor {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from DipoleInversion do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole_inversion[e41], dipole_inversion[e42], dipole_inversion[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12], dipole_inversion[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([dipole_inversion[e15], dipole_inversion[e25], dipole_inversion[e35], 0.0]),
        ));
    }
}

impl TryFrom<Flector> for AntiCircleRotor {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiCircleRotor {
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([multi_vector[e41], multi_vector[e42], multi_vector[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[scalar]]),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiCircleRotor {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from VersorOdd do not fit into AntiCircleRotor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from([versor_odd[e41], versor_odd[e42], versor_odd[e43]]),
            // e23, e31, e12, e45
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e45]]),
            // e15, e25, e35, scalar
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[scalar]]),
        ));
    }
}
