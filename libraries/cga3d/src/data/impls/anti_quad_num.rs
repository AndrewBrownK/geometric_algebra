use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 149
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       3       0
//  Average:         2       6       0
//  Maximum:        33      46       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       4       0
//  Average:         4      10       0
//  Maximum:        96     134       0
impl std::ops::Add<AntiCircleRotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (other.group2()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (other.group1()[3] + self.group0()[2])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum321> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum321) -> Self::Output {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            self.group0()[1],
            (other.group0()[0] + self.group0()[2]),
            (other.group0()[1] + self.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiDualNum321> for AntiQuadNum {
    fn add_assign(&mut self, other: AntiDualNum321) {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            self.group0()[1],
            (other.group0()[0] + self.group0()[2]),
            (other.group0()[1] + self.group0()[3]),
        ]));
        *self = addition;
    }
}
impl std::ops::Add<AntiDualNum4> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum4) -> Self::Output {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (other.group0()[0] + self.group0()[0]),
            self.group0()[1],
            self.group0()[2],
            (other.group0()[1] + self.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiDualNum4> for AntiQuadNum {
    fn add_assign(&mut self, other: AntiDualNum4) {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (other.group0()[0] + self.group0()[0]),
            self.group0()[1],
            self.group0()[2],
            (other.group0()[1] + self.group0()[3]),
        ]));
        *self = addition;
    }
}
impl std::ops::Add<AntiDualNum5> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiDualNum5) -> Self::Output {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            (other.group0()[0] + self.group0()[1]),
            self.group0()[2],
            (other.group0()[1] + self.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiDualNum5> for AntiQuadNum {
    fn add_assign(&mut self, other: AntiDualNum5) {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            (other.group0()[0] + self.group0()[1]),
            self.group0()[2],
            (other.group0()[1] + self.group0()[3]),
        ]));
        *self = addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for AntiQuadNum {
    type Output = VersorOdd;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] + self.group0()[1])]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiQuadNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiQuadNum) -> Self::Output {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ (other.group0() + self.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<AntiQuadNum> for AntiQuadNum {
    fn add_assign(&mut self, other: AntiQuadNum) {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ (other.group0() + self.group0()));
        *self = addition;
    }
}
impl std::ops::Add<AntiScalar> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiTripleNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiTripleNum) -> Self::Output {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (other.group0()[0] + self.group0()[0]),
            (other.group0()[1] + self.group0()[1]),
            self.group0()[2],
            (other.group0()[2] + self.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiTripleNum> for AntiQuadNum {
    fn add_assign(&mut self, other: AntiTripleNum) {
        let addition = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (other.group0()[0] + self.group0()[0]),
            (other.group0()[1] + self.group0()[1]),
            self.group0()[2],
            (other.group0()[2] + self.group0()[3]),
        ]));
        *self = addition;
    }
}
impl std::ops::Add<Circle> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[2] + other.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[2] + other.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group0()[1] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum321> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: DualNum321) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<DualNum4> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: DualNum4) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<DualNum5> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: DualNum5) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[2] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Flector) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[2] + other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[1] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<Line> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] + self.group0()[3]), other.group0()[1]]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e15, e25, e35, e45
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group0()[2] + other.group3()[3])]),
            // e41, e42, e43
            other.group4(),
            // e23, e31, e12
            other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group9()[0], other.group9()[1], other.group9()[2], (self.group0()[1] + other.group9()[3])]),
            // e1234
            (self.group0()[0] + other[e45]),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Plane) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<QuadNum> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: QuadNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[2]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for AntiQuadNum {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[scalar])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Sphere> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] + other[e4315])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<TripleNum> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: TripleNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[2]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiQuadNum {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[2] + other.group1()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], (self.group0()[0] + other.group2()[3])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], (self.group0()[1] + other.group3()[3])]),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        5       21        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum321> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        0
    fn bitxor(self, other: AntiDualNum321) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum321> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: AntiDualNum321) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum4> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        0
    fn bitxor(self, other: AntiDualNum4) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum4> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: AntiDualNum4) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum5> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        5        0
    fn bitxor(self, other: AntiDualNum5) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum5> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: AntiDualNum5) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiQuadNum {
    type Output = CircleRotor;
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
impl std::ops::BitXor<AntiFlector> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiQuadNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       14        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       11        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiQuadNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: AntiQuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiQuadNum> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: AntiQuadNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiScalar> for AntiQuadNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiTripleNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiTripleNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiTripleNum> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: AntiTripleNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiQuadNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiQuadNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       12        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiQuadNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       13        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiQuadNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       18        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum321> for AntiQuadNum {
    type Output = DualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: DualNum321) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum4> for AntiQuadNum {
    type Output = DualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: DualNum4) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum5> for AntiQuadNum {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: DualNum5) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiQuadNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiQuadNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiQuadNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiQuadNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd3        0        4        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       27        0
    //  no simd       12       44        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for AntiQuadNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<QuadNum> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn bitxor(self, other: QuadNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        1       13        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiQuadNum {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiQuadNum {
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
impl std::ops::BitXor<TripleNum> for AntiQuadNum {
    type Output = TripleNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        5        0
    fn bitxor(self, other: TripleNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        6       22        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       22        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiDualNum321> for AntiQuadNum {
    fn from(anti_dual_num321: AntiDualNum321) -> Self {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, 0.0, anti_dual_num321[e45], anti_dual_num321[scalar]]));
    }
}

impl From<AntiDualNum4> for AntiQuadNum {
    fn from(anti_dual_num4: AntiDualNum4) -> Self {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([anti_dual_num4[e1234], 0.0, 0.0, anti_dual_num4[scalar]]));
    }
}

impl From<AntiDualNum5> for AntiQuadNum {
    fn from(anti_dual_num5: AntiDualNum5) -> Self {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, anti_dual_num5[e3215], 0.0, anti_dual_num5[scalar]]));
    }
}

impl From<AntiTripleNum> for AntiQuadNum {
    fn from(anti_triple_num: AntiTripleNum) -> Self {
        use crate::elements::*;
        return AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([anti_triple_num[e1234], anti_triple_num[e3215], 0.0, anti_triple_num[scalar]]),
        );
    }
}

impl From<Scalar> for AntiQuadNum {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, 0.0, 0.0, scalar[scalar]]));
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd4        8        8        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       32       47        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       15        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       48       63        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum321> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn mul(self, other: AntiDualNum321) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum321> for AntiQuadNum {
    fn mul_assign(&mut self, other: AntiDualNum321) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum4> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        0
    fn mul(self, other: AntiDualNum4) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum4> for AntiQuadNum {
    fn mul_assign(&mut self, other: AntiDualNum4) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum5> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        0
    fn mul(self, other: AntiDualNum5) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiDualNum5> for AntiQuadNum {
    fn mul_assign(&mut self, other: AntiDualNum5) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        3       21        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        7       22        0
    //  no simd       16       40        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiQuadNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       12       24        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       23        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       10       27        0
    //  no simd       16       39        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1       12        0
    //  no simd        1       24        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiQuadNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        3        5        0
    // no simd       12       20        0
    fn mul(self, other: AntiQuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiQuadNum> for AntiQuadNum {
    fn mul_assign(&mut self, other: AntiQuadNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiTripleNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        4        0
    // no simd        8       16        0
    fn mul(self, other: AntiTripleNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiTripleNum> for AntiQuadNum {
    fn mul_assign(&mut self, other: AntiTripleNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       42        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       20        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       10       27        0
    //  no simd       31       48        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       41        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       20        0
    //    simd4       11       11        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       47       64        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum321> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn mul(self, other: DualNum321) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum4> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        0
    fn mul(self, other: DualNum4) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum5> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        8        0
    fn mul(self, other: DualNum5) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       20        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       16        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       36        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiQuadNum {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       12       24        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       16       36        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd2        3        5        0
    //    simd3       12       16        0
    //    simd4       12       17        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       96      134        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1       14        0
    //  no simd        1       23        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<QuadNum> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       16        0
    fn mul(self, other: QuadNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       26        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiQuadNum {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       23        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<TripleNum> for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: TripleNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiQuadNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       18        0
    // no simd       48       72        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4       12       18        0
    // no simd       48       72        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiQuadNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn neg(self) -> Self {
        let negation = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ (self.group0() * Simd32x4::from(-1.0)));
        return negation;
    }
}
impl std::ops::Not for AntiQuadNum {
    type Output = AntiQuadNum;
    fn not(self) -> Self::Output {
        return self;
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (-other.group2()[3] + self.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (-other.group1()[3] + self.group0()[2]),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum321> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiDualNum321) -> Self::Output {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            self.group0()[1],
            (-other.group0()[0] + self.group0()[2]),
            (-other.group0()[1] + self.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDualNum321> for AntiQuadNum {
    fn sub_assign(&mut self, other: AntiDualNum321) {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            self.group0()[1],
            (-other.group0()[0] + self.group0()[2]),
            (-other.group0()[1] + self.group0()[3]),
        ]));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDualNum4> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiDualNum4) -> Self::Output {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-other.group0()[0] + self.group0()[0]),
            self.group0()[1],
            self.group0()[2],
            (-other.group0()[1] + self.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDualNum4> for AntiQuadNum {
    fn sub_assign(&mut self, other: AntiDualNum4) {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-other.group0()[0] + self.group0()[0]),
            self.group0()[1],
            self.group0()[2],
            (-other.group0()[1] + self.group0()[3]),
        ]));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiDualNum5> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: AntiDualNum5) -> Self::Output {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            (-other.group0()[0] + self.group0()[1]),
            self.group0()[2],
            (-other.group0()[1] + self.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiDualNum5> for AntiQuadNum {
    fn sub_assign(&mut self, other: AntiDualNum5) {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            self.group0()[0],
            (-other.group0()[0] + self.group0()[1]),
            self.group0()[2],
            (-other.group0()[1] + self.group0()[3]),
        ]));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group0()[3] + self.group0()[3])]),
            // e23, e31, e12, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (-other.group1()[3] + self.group0()[1])]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiQuadNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: AntiQuadNum) -> Self::Output {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ (-other.group0() + self.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiQuadNum> for AntiQuadNum {
    fn sub_assign(&mut self, other: AntiQuadNum) {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ (-other.group0() + self.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiTripleNum> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn sub(self, other: AntiTripleNum) -> Self::Output {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-other.group0()[0] + self.group0()[0]),
            (-other.group0()[1] + self.group0()[1]),
            self.group0()[2],
            (-other.group0()[2] + self.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiTripleNum> for AntiQuadNum {
    fn sub_assign(&mut self, other: AntiTripleNum) {
        let subtraction = AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-other.group0()[0] + self.group0()[0]),
            (-other.group0()[1] + self.group0()[1]),
            self.group0()[2],
            (-other.group0()[2] + self.group0()[3]),
        ]));
        *self = subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[2] - other.group1()[3]),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       12        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[2] - other.group1()[3]),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group0()[1] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum321> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum321) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum4> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum4) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum5> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum5) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[2] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[2] - other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[1] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        4       28        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(-other.group0()[0] + self.group0()[3]), (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group0()[2] - other.group3()[3]),
            ]),
            // e41, e42, e43
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group9()[0] * -1.0),
                (other.group9()[1] * -1.0),
                (other.group9()[2] * -1.0),
                (self.group0()[1] - other.group9()[3]),
            ]),
            // e1234
            (self.group0()[0] - other[e45]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<QuadNum> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[2] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for AntiQuadNum {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[scalar])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Sphere> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] - other[e4315])]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<TripleNum> for AntiQuadNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: TripleNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[3], (other.group0()[2] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiQuadNum {
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
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[2]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1]]),
            // e1234
            self.group0()[0],
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiQuadNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group0()[3]),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[2] - other.group1()[3]),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] * -1.0),
                (other.group2()[1] * -1.0),
                (other.group2()[2] * -1.0),
                (self.group0()[0] - other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group3()[0] * -1.0),
                (other.group3()[1] * -1.0),
                (other.group3()[2] * -1.0),
                (self.group0()[1] - other.group3()[3]),
            ]),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleRotor> for AntiQuadNum {
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
        let el = anti_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
            let mut error = "Elements from AntiCircleRotor do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([0.0, 0.0, anti_circle_rotor[e45], anti_circle_rotor[scalar]]),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiQuadNum {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([0.0, anti_motor[e3215], 0.0, anti_motor[scalar]]),
        ));
    }
}

impl TryFrom<Dipole> for AntiQuadNum {
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
        let el = dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
            let mut error = "Elements from Dipole do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, 0.0, dipole[e45], 0.0])));
    }
}

impl TryFrom<DipoleInversion> for AntiQuadNum {
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
        let el = dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([dipole_inversion[e1234], dipole_inversion[e3215], dipole_inversion[e45], 0.0]),
        ));
    }
}

impl TryFrom<FlatPoint> for AntiQuadNum {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flat_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flat_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, 0.0, flat_point[e45], 0.0])));
    }
}

impl TryFrom<Flector> for AntiQuadNum {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
        if fail {
            let mut error = "Elements from Flector do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, flector[e3215], flector[e45], 0.0])));
    }
}

impl TryFrom<MultiVector> for AntiQuadNum {
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
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            multi_vector[e1234], multi_vector[e3215], multi_vector[e45], multi_vector[scalar],
        ])));
    }
}

impl TryFrom<Plane> for AntiQuadNum {
    type Error = String;
    fn try_from(plane: Plane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Plane do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([0.0, plane[e3215], 0.0, 0.0])));
    }
}

impl TryFrom<Sphere> for AntiQuadNum {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([sphere[e1234], sphere[e3215], 0.0, 0.0])));
    }
}

impl TryFrom<VersorOdd> for AntiQuadNum {
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
        let el = versor_odd[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiQuadNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiQuadNum::from_groups(
            // e1234, e3215, e45, scalar
            Simd32x4::from([versor_odd[e1234], versor_odd[e3215], versor_odd[e45], versor_odd[scalar]]),
        ));
    }
}
