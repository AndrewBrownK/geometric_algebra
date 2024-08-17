use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 71
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         9      12       0
//  Maximum:       106     133       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       0       0
//   Median:         1       4       0
//  Average:        15      20       0
//  Maximum:       225     259       0
impl std::ops::Add<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e12345])]),
            // e235, e315, e125, e5
            self.group1(),
        );
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for Motor {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other[e12345])]),
            // e235, e315, e125, e5
            self.group1(),
        );
        *self = addition;
    }
}
impl std::ops::Add<Circle> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Circle) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        0        0
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group2()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] + other.group2()[0]),
                (self.group1()[1] + other.group2()[1]),
                (self.group1()[2] + other.group2()[2]),
                self.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for Motor {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]),
            // e41, e42, e43
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for Motor {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]),
            // e41, e42, e43
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other.group2()[3],
            // e12, e31, e23
            Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[1])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<DualNum> for Motor {
    fn add_assign(&mut self, other: DualNum) {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] + other.group0()[1])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other.group0()[0])]),
        );
        *self = addition;
    }
}
impl std::ops::Add<FlatPoint> for Motor {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for Motor {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            other.group1(),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Line) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
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
                self.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Line> for Motor {
    fn add_assign(&mut self, other: Line) {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
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
                self.group1()[3],
            ]),
        );
        *self = addition;
    }
}
impl std::ops::Add<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() + other.group0()),
            // e235, e315, e125, e5
            (self.group1() + other.group1()),
        );
        return addition;
    }
}
impl std::ops::AddAssign<Motor> for Motor {
    fn add_assign(&mut self, other: Motor) {
        let addition = Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() + other.group0()),
            // e235, e315, e125, e5
            (self.group1() + other.group1()),
        );
        *self = addition;
    }
}
impl std::ops::Add<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        0        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        6        0        0
    //  no simd        8        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], (self.group0()[3] + other.group0()[1])]),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            (self.group1()[3] + other[e1]),
            // e15, e25, e35, e45
            other.group3(),
            // e41, e42, e43
            other.group4(),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group5()[0]),
                (self.group0()[1] + other.group5()[1]),
                (self.group0()[2] + other.group5()[2]),
                other.group5()[3],
            ]),
            // e423, e431, e412
            other.group6(),
            // e235, e315, e125
            (other.group7() + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e4235, e4315, e4125, e3215
            other.group8(),
            // e1234
            other[e35],
            // e12, e31, e23
            other.group10(),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for Motor {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] + other[e2])]),
            // e1, e2, e3, e4
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for Motor {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Sphere> for Motor {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
            // e1234
            other[e4315],
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd        8        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[3] + other.group0()[3])]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e235, e315, e125, e5
            (self.group1() + other.group2()),
            // e1, e2, e3, e4
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for Motor {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]),
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            other.group3(),
            // e1234
            other.group2()[3],
            // e12, e31, e23
            Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]),
        );
        return addition;
    }
}
impl std::ops::BitXor<Circle> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Dipole> for Motor {
    fn bitxor_assign(&mut self, other: Dipole) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       13        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DipoleInversion> for Motor {
    fn bitxor_assign(&mut self, other: DipoleInversion) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       25       45        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for Motor {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<VersorOdd> for Motor {
    fn bitxor_assign(&mut self, other: VersorOdd) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for Motor {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, anti_scalar[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for Motor {
    fn from(dual_num: DualNum) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
        );
    }
}

impl From<Line> for Motor {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
        );
    }
}
impl std::ops::Mul<AntiScalar> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       16        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       55        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       43       62        0
    //  no simd       64       83        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       55        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       48       64        0
    //  no simd       75       91        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       64       80        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4       22       22        0
    // Totals...
    // yes simd       42       54        0
    //  no simd      108      120        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       16        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       12       24        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       40       48        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       48        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       59       83        0
    //    simd3       22       24        0
    //    simd4       25       26        0
    // Totals...
    // yes simd      106      133        0
    //  no simd      225      259        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       20        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       14       22        0
    //  no simd       20       28        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       52        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Motor {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       62        0
    //    simd4       18       18        0
    // Totals...
    // yes simd       58       80        0
    //  no simd      112      134        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4       24       25        0
    // Totals...
    // yes simd       41       53        0
    //  no simd      113      128        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for Motor {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn neg(self) -> Self {
        let negation = Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            (self.group1() * Simd32x4::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn not(self) -> Self::Output {
        let dual = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * -1.0)]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * -1.0)]),
        );
        return dual;
    }
}
impl std::ops::Sub<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e12345])]),
            // e235, e315, e125, e5
            self.group1(),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for Motor {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other[e12345])]),
            // e235, e315, e125, e5
            self.group1(),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Circle> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        4        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        4        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] - other.group2()[0]),
                (self.group1()[1] - other.group2()[1]),
                (self.group1()[2] - other.group2()[2]),
                self.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for Motor {
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
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            (Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for Motor {
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
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
            // e1234
            (other.group2()[3] * -1.0),
            // e12, e31, e23
            (Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[1])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<DualNum> for Motor {
    fn sub_assign(&mut self, other: DualNum) {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] - other.group0()[1])]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other.group0()[0])]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            (other.group1() * Simd32x4::from(-1.0)),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
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
                self.group1()[3],
            ]),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for Motor {
    fn sub_assign(&mut self, other: Line) {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
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
                self.group1()[3],
            ]),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() - other.group0()),
            // e235, e315, e125, e5
            (self.group1() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::SubAssign<Motor> for Motor {
    fn sub_assign(&mut self, other: Motor) {
        let subtraction = Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() - other.group0()),
            // e235, e315, e125, e5
            (self.group1() - other.group1()),
        );
        *self = subtraction;
    }
}
impl std::ops::Sub<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        3        0
    //    simd3        1        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6        9        0
    //  no simd        8       24        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), (self.group0()[3] - other.group0()[1])]),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (self.group1()[3] - other[e1]),
            // e15, e25, e35, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (other.group4() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group5()[0]),
                (self.group0()[1] - other.group5()[1]),
                (self.group0()[2] - other.group5()[2]),
                (other.group5()[3] * -1.0),
            ]),
            // e423, e431, e412
            (other.group6() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (-other.group7() + Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e4235, e4315, e4125, e3215
            (other.group8() * Simd32x4::from(-1.0)),
            // e1234
            (other[e35] * -1.0),
            // e12, e31, e23
            (other.group10() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        1        4        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] - other[e2])]),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for Motor {
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
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for Motor {
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
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
            // e1234
            (other[e4315] * -1.0),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        5        0
    //  no simd        8        8        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[3] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            (self.group1() - other.group2()),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for Motor {
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
            Simd32x4::from(0.0),
            // e5
            self.group1()[3],
            // e15, e25, e35, e45
            (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
            // e1234
            (other.group2()[3] * -1.0),
            // e12, e31, e23
            (Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]) * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}

impl TryFrom<Circle> for Motor {
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
        let el = circle[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle[e415], circle[e425], circle[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleRotor> for Motor {
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
        let el = circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleRotor do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle_rotor[e415], circle_rotor[e425], circle_rotor[e435], circle_rotor[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_rotor[e235], circle_rotor[e315], circle_rotor[e125], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for Motor {
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
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<RoundPoint> for Motor {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
        ));
    }
}

impl TryFrom<VersorEven> for Motor {
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
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
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
            let mut error = "Elements from VersorEven do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e5]]),
        ));
    }
}
